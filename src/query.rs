#![allow(dead_code)]
use chitin::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum UserDetailQuery {
    #[chitin(request, response = "(crate::model::User, String)")]
    AskUserDetail { user_id: i32 },
}

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum UserQuery {
    #[chitin(request, response = "Vec<crate::model::Article>")]
    AskUserArticles { user_id: i32, count: usize },
    #[chitin(router)]
    UserDetail(UserDetailQuery),
}

#[derive(Serialize, Deserialize, ChitinCodegen)]
pub enum RootQuery {
    #[chitin(request, response = "Vec<crate::model::Article>")]
    AskArticles { count: usize },
    #[chitin(request, response = "()")]
    PostArticle { article: crate::model::Article },
    #[chitin(router)]
    User(UserQuery), // 注意：假如這裡打錯成 `User(i32)` 或其它不是 `ChitinCodegen` 的東西，會報出很難解的錯誤
}
