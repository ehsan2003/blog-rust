use std::error::Error;

#[derive(Debug)]
pub enum ApplicationException {
    NotFoundException(String),
    DuplicationException {
        key: String,
        value: String,
    },
    ValidationException {
        key: String,
        value: String,
        message: String,
    },
    InternalException(UnknownException),
    ForBiddenException(String),
}

pub type UnknownException = Box<dyn std::error::Error + Send + Sync>;

pub type ApplicationResult<T> = Result<T, ApplicationException>;
pub type UnknownResult<T> = Result<T, UnknownException>;

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
