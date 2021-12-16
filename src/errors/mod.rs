use std::error::Error;

pub mod auth_payload_id_not_found;
pub mod validation;

#[derive(Debug)]
pub enum ApplicationException {
    NotFoundException(String),
    BadRequestException(String),
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
