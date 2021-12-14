use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::utils::CryptoService;

pub const HASH_RESULT: &str = "hash_result";
pub struct CryptoServiceSpy {
    verify_result: bool,
    pub hash_called_with: Mutex<Vec<String>>,
    pub verify_called_with: Mutex<Vec<(String, String)>>,
}

impl CryptoServiceSpy {
    pub fn new_verified() -> CryptoServiceSpy {
        CryptoServiceSpy {
            verify_result: true,
            hash_called_with: Mutex::new(Vec::new()),
            verify_called_with: Mutex::new(Vec::new()),
        }
    }
    pub fn new_unverified() -> CryptoServiceSpy {
        CryptoServiceSpy {
            verify_result: false,
            hash_called_with: Mutex::new(Vec::new()),
            verify_called_with: Mutex::new(Vec::new()),
        }
    }

    pub fn assert_hash_calls(&self, expected: Vec<String>) {
        let mut called_with = self.hash_called_with.lock().unwrap();
        assert_eq!(*called_with, expected);
    }
    pub fn assert_verify_calls(&self, expected: Vec<(String, String)>) {
        let mut called_with = self.verify_called_with.lock().unwrap();
        assert_eq!(*called_with, expected);
    }
}
#[async_trait::async_trait]
impl CryptoService for CryptoServiceSpy {
    async fn hash(&self, _data: &str) -> UnknownResult<String> {
        self.hash_called_with
            .lock()
            .unwrap()
            .push(_data.to_string());
        Ok(HASH_RESULT.to_string())
    }

    async fn verify(&self, _data: &str, _hash: &str) -> UnknownResult<bool> {
        self.verify_called_with
            .lock()
            .unwrap()
            .push((_data.to_string(), _hash.to_string()));
        Ok(self.verify_result)
    }
}
