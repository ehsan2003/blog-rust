use std::sync::Mutex;

use crate::access_management::Role;
use crate::utils::AuthPayload;

#[derive(Debug)]
pub struct DisallowedAuthPayloadSpy {
    pub called_with: Mutex<Vec<String>>,
}

impl Clone for DisallowedAuthPayloadSpy {
    fn clone(&self) -> Self {
        DisallowedAuthPayloadSpy {
            called_with: Mutex::new(self.called_with.lock().unwrap().clone()),
        }
    }
}

impl Role for DisallowedAuthPayloadSpy {
    fn can(&self, action: &str) -> bool {
        self.called_with.lock().unwrap().push(action.to_string());
        false
    }
}

impl AuthPayload for DisallowedAuthPayloadSpy {
    fn get_user_id(&self) -> String {
        "WEAK".to_string()
    }
}

impl DisallowedAuthPayloadSpy {
    pub fn new() -> Self {
        DisallowedAuthPayloadSpy {
            called_with: Mutex::new(Vec::new()),
        }
    }
}
