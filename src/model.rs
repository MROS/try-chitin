use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Article {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub sentence: String,
}
