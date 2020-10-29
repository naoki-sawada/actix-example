use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NewPost, Post, UpdatePost};

pub fn find_post_by_uid(
    uid: Uuid,
    conn: &PgConnection,
) -> Result<Option<Post>, diesel::result::Error> {
    use crate::schema::posts::dsl::{id, posts};
    let post = posts
        .filter(id.eq(uid.to_string()))
        .first::<Post>(conn)
        .optional()?;
    Ok(post)
}

pub fn add_post(post: &NewPost, conn: &PgConnection) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts::dsl::posts;
    let new_post = Post {
        id: Uuid::new_v4().to_string(),
        title: post.title.clone(),
        body: post.body.clone(),
        published: post.published,
    };
    diesel::insert_into(posts).values(&new_post).execute(conn)?;
    Ok(new_post)
}

pub fn update_post(
    uid: Uuid,
    post: &UpdatePost,
    conn: &PgConnection,
) -> Result<Post, diesel::result::Error> {
    use crate::schema::posts::dsl::{id, posts};
    let updated_post = diesel::update(posts.filter(id.eq(uid.to_string())))
        .set(post)
        .get_result::<Post>(conn)?;
    Ok(updated_post)
}

pub fn delete_post(uid: Uuid, conn: &PgConnection) -> Result<Uuid, diesel::result::Error> {
    use crate::schema::posts::dsl::{id, posts};
    diesel::delete(posts.filter(id.eq(uid.to_string()))).execute(conn)?;
    Ok(uid)
}
