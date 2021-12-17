use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::users::domain::User;
use crate::utils::{AuthPayload, AuthPayloadResolver};

pub struct AuthPayloadResolverSpy {
    pub payload_ids: Mutex<Vec<String>>,
    pub returning_user: User,
}
#[async_trait::async_trait]
impl AuthPayloadResolver for AuthPayloadResolverSpy {
    async fn resolve(&self, auth_payload: &(dyn AuthPayload)) -> UnknownResult<User> {
        self.payload_ids
            .lock()
            .unwrap()
            .push(auth_payload.get_user_id());
        Ok(self.returning_user.clone())
    }
}

impl AuthPayloadResolverSpy {
    pub fn new_returning(returning_user: User) -> Self {
        Self {
            payload_ids: Mutex::new(Vec::new()),
            returning_user,
        }
    }
}
