use crate::errors::UnknownResult;

#[async_trait::async_trait]
pub trait CryptoService:Send + Sync {
    async fn hash(&self, data: &str) -> UnknownResult<String>;
    async fn verify(&self, data: &str, hash: &str) -> UnknownResult<bool>;
}