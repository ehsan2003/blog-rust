#[async_trait::async_trait]
pub trait RandomService {
    async fn secure_random_password() -> String;
    async fn random_id() -> String;
}