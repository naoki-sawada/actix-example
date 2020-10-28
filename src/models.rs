use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}
