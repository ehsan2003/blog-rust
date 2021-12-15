use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::utils::RandomService;

pub struct RandomServiceSpy {
    pub secure_random_called: Mutex<bool>,
    pub random_id_called: Mutex<bool>,
}
pub const SECURE_RANDOM_PASSWORD: &str = "password";
pub const RANDOM_ID: &str = "random id";

#[async_trait::async_trait]
impl RandomService for RandomServiceSpy {
    async fn secure_random_password(&self) -> UnknownResult<String> {
        *self.secure_random_called.lock().unwrap() = true;
        Ok(SECURE_RANDOM_PASSWORD.to_string())
    }

    async fn random_id(&self) -> UnknownResult<String> {
        *self.random_id_called.lock().unwrap() = true;
        Ok(RANDOM_ID.to_string())
    }
}

impl RandomServiceSpy {
    pub fn new() -> Self {
        RandomServiceSpy {
            secure_random_called: Mutex::new(false),
            random_id_called: Mutex::new(false),
        }
    }

    pub fn assert_secure_random_called(&self) {
        assert!(*self.secure_random_called.lock().unwrap());
    }
    pub fn assert_random_id_called(&self) {
        assert!(*self.random_id_called.lock().unwrap());
    }
}
