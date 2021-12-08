pub enum ApplicationException {
    NotFoundException(String),
    DuplicationException { key: String, value: String },
    ValidationException { key: String, value: String, message: String },
    InternalException(UnknownException),
}

pub type UnknownException = Box<dyn std::error::Error + Send + Sync>;

pub type ApplicationResult<T> = Result<T, ApplicationException>;
pub type UnknownResult<T> = Result<T, UnknownException>;
