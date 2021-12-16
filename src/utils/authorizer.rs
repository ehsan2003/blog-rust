use crate::errors::UnknownResult;
use crate::users::domain::User;

#[async_trait::async_trait]
pub trait Authorizer: Send + Sync {
    async fn authorize(&self, user: &User, password: &str) -> UnknownResult<bool>;
}
