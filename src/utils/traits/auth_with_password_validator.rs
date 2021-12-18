use crate::errors::UnknownResult;
use crate::utils::AuthPayload;

#[async_trait::async_trait]
pub trait AuthWithPasswordValidator {
    async fn validate(&self, auth: &(dyn AuthPayload), password: &str) -> UnknownResult<bool>;
}
