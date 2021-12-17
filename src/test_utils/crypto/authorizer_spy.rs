use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::users::domain::User;
use crate::utils::Authorizer;

pub struct AuthorizerSpy {
    is_authorized: bool,
    calls: Mutex<Vec<(User, String)>>,
}
#[async_trait::async_trait]
impl Authorizer for AuthorizerSpy {
    async fn authorize(&self, user: &User, password: &str) -> UnknownResult<bool> {
        self.calls
            .lock()
            .unwrap()
            .push((user.clone(), password.to_string()));
        Ok(self.is_authorized)
    }
}

impl AuthorizerSpy {
    pub fn new_authorized() -> Self {
        Self {
            is_authorized: true,
            calls: Mutex::new(Vec::new()),
        }
    }
    pub fn new_unauthorized() -> Self {
        Self {
            is_authorized: false,
            calls: Mutex::new(Vec::new()),
        }
    }
    pub fn get_calls(&self) -> Vec<(User, String)> {
        self.calls.lock().unwrap().clone()
    }
}
