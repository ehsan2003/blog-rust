use std::error::Error;

use crate::errors::ApplicationException;

pub struct ValidationError {
    pub key: String,
    pub value: String,
    pub message: String,
}

impl ValidationError {
    pub fn new(key: String, value: String, message: String) -> Self {
        Self {
            key,
            value,
            message,
        }
    }
}

impl From<ValidationError> for ApplicationException {
    fn from(error: ValidationError) -> Self {
        ApplicationException::ValidationException {
            key: error.key,
            value: error.value,
            message: error.message,
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ApplicationException {
    fn from(error: Box<dyn Error + Send + Sync>) -> Self {
        ApplicationException::InternalException(error)
    }
}
