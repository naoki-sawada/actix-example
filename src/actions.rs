use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{Post,NewPost};

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

pub fn add_post(
    post: &NewPost,
    conn: &PgConnection,
) -> Result<Post, diesel::result::Error> {
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
