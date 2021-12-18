use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::utils::{AuthPayload, AuthWithPasswordValidator};

pub struct AuthWithPasswordValidatorSpy {
    pub called_with: Mutex<Vec<(String, String)>>,
    pub result: bool,
}
#[async_trait::async_trait]
impl AuthWithPasswordValidator for AuthWithPasswordValidatorSpy {
    async fn validate(&self, auth: &(dyn AuthPayload), password: &str) -> UnknownResult<bool> {
        self.called_with
            .lock()
            .unwrap()
            .push((auth.get_user_id(), password.to_string()));
        Ok(self.result)
    }
}

impl AuthWithPasswordValidatorSpy {
    pub fn new_verified() -> Self {
        Self {
            result: true,
            called_with: Mutex::new(Vec::new()),
        }
    }
    pub fn new_unverified() -> Self {
        Self {
            result: false,
            called_with: Mutex::new(Vec::new()),
        }
    }
    pub fn get_called_with(&self) -> Vec<(String, String)> {
        self.called_with.lock().unwrap().clone()
    }
}
