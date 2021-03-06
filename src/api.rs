use crate::api_trait::*;
// use crate::query::*;
use async_trait::async_trait;

#[derive(Default)]
pub struct UserDetail {}

#[async_trait]
impl UserDetailQueryRouter for UserDetail {
    async fn ask_user_detail(&self, user_id: i32) -> Result<crate::model::User, String> {
        let users = crate::USERS.lock().unwrap();
        match users.get(&user_id) {
            Some(user) => Ok(user.clone()),
            None => Err("無此 id".to_owned()),
        }
    }
}

#[derive(Default)]
pub struct UserQuery {
    user_detail: UserDetail,
}

#[async_trait]
impl UserQueryRouter for UserQuery {
    type UserDetailQueryRouter = UserDetail;
    async fn ask_user_articles(
        &self,
        count: usize,
        user_id: i32,
    ) -> Result<Vec<crate::model::Article>, String> {
        let articles = crate::ARTICLES.lock().unwrap();
        Ok(articles
            .iter()
            .filter(|a| a.author_id == user_id)
            .take(count)
            .cloned()
            .collect())
    }
    fn user_detail_router(&self) -> &Self::UserDetailQueryRouter {
        &self.user_detail
    }
}

#[derive(Default)]
pub struct RootQuery {
    user_query: UserQuery,
}

#[async_trait]
impl RootQueryRouter for RootQuery {
    type UserQueryRouter = UserQuery;
    async fn ask_articles(&self, count: usize) -> Result<Vec<crate::model::Article>, String> {
        let articles = crate::ARTICLES.lock().unwrap();
        Ok(articles.iter().take(count).cloned().collect())
    }
    async fn post_article(&self, article: crate::model::Article) -> Result<(), String> {
        let mut articles = crate::ARTICLES.lock().unwrap();
        articles.push(article);
        Ok(())
    }
    async fn create_user(&self, user: crate::model::User) -> Result<i32, String> {
        let mut users = crate::USERS.lock().unwrap();
        let id = users.len() as i32;
        users.insert(id, user);
        Ok(id)
    }
    fn user_router(&self) -> &Self::UserQueryRouter {
        &self.user_query
    }
}
