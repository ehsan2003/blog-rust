use crate::errors::UnknownResult;
use crate::users::domain::User;

#[async_trait::async_trait]
pub trait UsersRepository: Send + Sync {
    async fn get_by_id(&self, id: &str) -> UnknownResult<Option<User>>;
    async fn get_by_email(&self, email: &str) -> UnknownResult<Option<User>>;
    async fn create(&self, user: &User) -> UnknownResult<()>;
    async fn update(&self, user: &User) -> UnknownResult<()>;
    async fn delete(&self, id: &str) -> UnknownResult<()>;

    async fn email_exists(&self, email: &str) -> UnknownResult<bool> {
        Ok(self.get_by_email(email).await?.is_some())
    }
    async fn id_exists(&self, id: &str) -> UnknownResult<bool> {
        Ok(self.get_by_id(id).await?.is_some())
    }
}
