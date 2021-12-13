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
}

pub type UnknownException = Box<dyn std::error::Error + Send + Sync>;

pub type ApplicationResult<T> = Result<T, ApplicationException>;
pub type UnknownResult<T> = Result<T, UnknownException>;

pub struct ValidationError {
    pub key: String,
    pub value: String,
    pub message: String,
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