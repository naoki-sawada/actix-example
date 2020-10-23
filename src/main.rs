mod auth;

use actix_cors::Cors;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use env_logger;
use log::info;
use serde::Deserialize;

#[derive(Deserialize)]
struct EchoRequest {
    name: String,
}

#[get("/")]
async fn hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[post("/echo")]
async fn echo(req: web::Json<EchoRequest>) -> Result<HttpResponse, Error> {
    let res: String = format!("Hello, {}.", req.name);
    info!("{}", res);
    Ok(HttpResponse::Ok().body(res))
}

#[get("/hey")]
async fn manual_hello() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hey there!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default().send_wildcard();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(HttpAuthentication::bearer(auth::validator))
            .wrap(cors)
            .service(hello)
            .service(echo)
            .service(manual_hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
