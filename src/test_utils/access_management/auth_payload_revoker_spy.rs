use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::utils::{AuthPayload, AuthPayloadRevoker};

pub struct AuthPayloadRevokerSpy {
    pub payload_ids: Mutex<Vec<String>>,
    pub revoked_ids: Mutex<Vec<String>>,
}

#[async_trait::async_trait]
impl AuthPayloadRevoker for AuthPayloadRevokerSpy {
    async fn revoke_auth_payload(&self, auth_payload: &(dyn AuthPayload)) -> UnknownResult<()> {
        self.payload_ids
            .lock()
            .unwrap()
            .push(auth_payload.get_user_id().to_string());
        Ok(())
    }

    async fn revoke_all_with_id(&self, id: &str) -> UnknownResult<()> {
        self.revoked_ids.lock().unwrap().push(id.to_string());
        Ok(())
    }
}

impl AuthPayloadRevokerSpy {
    pub fn get_payload_ids(&self) -> Vec<String> {
        self.payload_ids.lock().unwrap().clone()
    }
    pub fn get_revoked_ids(&self) -> Vec<String> {
        self.revoked_ids.lock().unwrap().clone()
    }
}
