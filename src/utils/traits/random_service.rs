use crate::errors::UnknownResult;

#[async_trait::async_trait]
pub trait RandomService: Send + Sync {
    async fn secure_random_password(&self) -> UnknownResult<String>;
    async fn random_id(&self) -> UnknownResult<String>;
}
