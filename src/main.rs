#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use env_logger;
use log::info;
use serde::Deserialize;
use uuid::Uuid;

mod actions;
mod auth;
mod models;
mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
struct EchoRequest {
    name: String,
}

#[get("/")]
async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[get("/hey")]
async fn manual_hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hey there!"))
}

async fn get_post(id: web::Path<String>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let uid = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(err) => {
            let res = HttpResponse::BadRequest().body(format!("Invalid UUID: {}", err));
            return Ok(res);
        }
    };

    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            let res = HttpResponse::InternalServerError()
                .body(format!("couldn't get db connection from pool: {}", err));
            return Ok(res);
        }
    };

    let post = web::block(move || actions::find_post_by_uid(uid, &conn)).await;

    match post {
        Ok(post) => {
            if let Some(post) = post {
                Ok(HttpResponse::Ok().json(post))
            } else {
                let res = HttpResponse::NotFound().body(format!("No post found with id: {}", id));
                Ok(res)
            }
        }
        Err(err) => {
            log::info!("{}", err);
            let res = HttpResponse::InternalServerError().body(format!("InternalServerError"));
            Ok(res)
        }
    }
}

async fn create_post(
    req: web::Json<models::NewPost>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            let res = HttpResponse::InternalServerError()
                .body(format!("couldn't get db connection from pool: {}", err));
            return Ok(res);
        }
    };

    let post = web::block(move || actions::add_post(&req, &conn)).await;

    match post {
        Ok(post) => Ok(HttpResponse::Ok().json(post)),
        Err(err) => {
            log::info!("{}", err);
            let res = HttpResponse::InternalServerError().body(format!("InternalServerError"));
            Ok(res)
        }
    }
}

async fn echo(req: web::Json<EchoRequest>) -> Result<HttpResponse, Error> {
    let res: String = format!("Hello, {}.", req.name);
    info!("{}", res);
    Ok(HttpResponse::Ok().body(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default().send_wildcard();
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(
                web::scope("/echo")
                    .wrap(HttpAuthentication::bearer(auth::validator))
                    .route("", web::post().to(echo)),
            )
            .service(
                web::scope("/post")
                    .route("/{id}", web::get().to(get_post))
                    .route("", web::post().to(create_post)),
                // .route("/{id}", web::get().to(get_post))
                // .route("/{id}", web::put().to(update_post))
                // .route("/{id}", web::delete().to(delete_post)),
            )
            .service(hello)
            .service(manual_hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
