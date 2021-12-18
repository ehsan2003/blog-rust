use ApplicationException::BadRequestException;

use crate::errors::{ApplicationException, ApplicationResult, UnknownResult};
use crate::utils::AuthPayload;

#[async_trait::async_trait]
pub trait AuthWithPasswordValidator: Send + Sync {
    async fn validate(&self, auth: &(dyn AuthPayload), password: &str) -> UnknownResult<bool>;
    async fn validate_or_fail(
        &self,
        auth: &(dyn AuthPayload),
        password: &str,
    ) -> ApplicationResult<()> {
        match self.validate(auth, password).await? {
            true => Ok(()),
            false => Err(BadRequestException("invalid password".into())),
        }
    }
}
