use diesel::prelude::*;
use uuid::Uuid;

use crate::models::Post;

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
