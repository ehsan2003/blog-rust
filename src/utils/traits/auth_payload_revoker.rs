use crate::errors::UnknownResult;
use crate::utils::AuthPayload;

#[async_trait::async_trait]
pub trait AuthPayloadRevoker {
    async fn revoke_auth_payload(&self, auth_payload: &(dyn AuthPayload)) -> UnknownResult<()>;
    async fn revoke_all_with_id(&self, id: &str) -> UnknownResult<()>;
}
