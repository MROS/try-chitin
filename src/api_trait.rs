use async_trait::async_trait;
use crate::query::*;
use serde_json::error::Error;
#[async_trait]
pub trait UserDetailQueryRouter {
    async fn ask_user_detail(&self, user_id: i32) -> Result<crate::model::User, String>;
    async fn handle(&self, query: UserDetailQuery) -> Result<String, Error> {
        match query {
             UserDetailQuery::AskUserDetail { user_id } => {
                 let resp = self.ask_user_detail(user_id).await;
                 serde_json::to_string(&resp)
             }
        }
    }
}
#[async_trait]
pub trait UserQueryRouter {
    type UserDetailQueryRouter: UserDetailQueryRouter + Sync;
    async fn ask_user_articles(&self, count: usize, user_id: i32) -> Result<Vec<crate::model::Article>, String>;
    fn user_detail_router(&self) -> &Self::UserDetailQueryRouter;
    async fn handle(&self, query: UserQuery) -> Result<String, Error> {
        match query {
             UserQuery::AskUserArticles { count, user_id } => {
                 let resp = self.ask_user_articles(count, user_id).await;
                 serde_json::to_string(&resp)
             }
             UserQuery::UserDetail(query) => {
                 self.user_detail_router().handle(query).await
             }
        }
    }
}
#[async_trait]
pub trait RootQueryRouter {
    type UserQueryRouter: UserQueryRouter + Sync;
    async fn ask_articles(&self, count: usize) -> Result<Vec<crate::model::Article>, String>;
    async fn post_article(&self, article: crate::model::Article) -> Result<(), String>;
    async fn create_user(&self, user: crate::model::User) -> Result<i32, String>;
    fn user_router(&self) -> &Self::UserQueryRouter;
    async fn handle(&self, query: RootQuery) -> Result<String, Error> {
        match query {
             RootQuery::AskArticles { count } => {
                 let resp = self.ask_articles(count).await;
                 serde_json::to_string(&resp)
             }
             RootQuery::PostArticle { article } => {
                 let resp = self.post_article(article).await;
                 serde_json::to_string(&resp)
             }
             RootQuery::CreateUser { user } => {
                 let resp = self.create_user(user).await;
                 serde_json::to_string(&resp)
             }
             RootQuery::User(query) => {
                 self.user_router().handle(query).await
             }
        }
    }
}
