use std::fmt::Debug;
use std::sync::Mutex;

use crate::access_management::Role;
use crate::utils::AuthPayload;

#[derive(Debug)]
pub struct AllowedAuthPayloadSpy {
    pub called_with: Mutex<Vec<String>>,
}

impl Clone for AllowedAuthPayloadSpy {
    fn clone(&self) -> Self {
        Self {
            called_with: Mutex::new(self.called_with.lock().unwrap().clone()),
        }
    }
}

impl AllowedAuthPayloadSpy {
    pub fn new() -> Self {
        Self {
            called_with: Mutex::new(Vec::new()),
        }
    }

    pub fn called_with(&self) -> Vec<String> {
        self.called_with.lock().unwrap().clone()
    }
}

impl Role for AllowedAuthPayloadSpy {
    fn can(&self, action: &str) -> bool {
        self.called_with.lock().unwrap().push(action.to_string());
        true
    }
}
pub const ALLOWED_AUTH_PAYLOAD_ID: &str = "ALLOWED_ID";

impl AuthPayload for AllowedAuthPayloadSpy {
    fn get_user_id(&self) -> String {
        ALLOWED_AUTH_PAYLOAD_ID.to_string()
    }
}
