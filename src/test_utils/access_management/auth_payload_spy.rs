use std::fmt::{Debug, Formatter};
use std::sync::Mutex;

use crate::access_management::Role;
use crate::utils::AuthPayload;

#[derive(Debug)]
pub struct AuthPayloadSpy {
    returning_id: String,
    can: bool,
    called_with: Mutex<Vec<String>>,
}

impl Clone for AuthPayloadSpy {
    fn clone(&self) -> Self {
        AuthPayloadSpy {
            returning_id: self.returning_id.clone(),
            can: self.can,
            called_with: Mutex::new(Vec::new()),
        }
    }
}
impl Role for AuthPayloadSpy {
    fn can(&self, action: &str) -> bool {
        self.called_with.lock().unwrap().push(action.to_string());
        self.can
    }
}

impl AuthPayload for AuthPayloadSpy {
    fn get_user_id(&self) -> String {
        self.returning_id.clone()
    }
}

impl AuthPayloadSpy {
    pub fn new_allowed(returning_id: String) -> Self {
        Self {
            returning_id,
            can: true,
            called_with: Mutex::new(Vec::new()),
        }
    }
    pub fn new_disallowed(returning_id: String) -> Self {
        Self {
            returning_id,
            can: false,
            called_with: Mutex::new(Vec::new()),
        }
    }

    pub fn get_called(&self) -> Vec<String> {
        self.called_with.lock().unwrap().clone()
    }
}
