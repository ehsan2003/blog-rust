use crate::errors::UnknownResult;
use crate::users::domain::User;
use crate::utils::AuthPayload;

#[async_trait::async_trait]
pub trait AuthPayloadResolver {
    async fn resolve(&self, auth_payload: &(dyn AuthPayload)) -> UnknownResult<User>;
}
