use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "posts"]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}
