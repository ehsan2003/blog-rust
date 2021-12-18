use crate::errors::{ApplicationException, ApplicationResult, UnknownResult};
use crate::users::domain::User;

#[async_trait::async_trait]
pub trait Authorizer: Send + Sync {
    async fn authorize(&self, user: &User, password: &str) -> UnknownResult<bool>;
    async fn authorize_or_fail(&self, user: &User, password: &str) -> ApplicationResult<()> {
        match self.authorize(user, password).await {
            Ok(true) => Ok(()),
            Ok(false) => Err(ApplicationException::BadRequestException(
                "invalid password".into(),
            )),
            Err(e) => Err(e)?,
        }
    }
}
