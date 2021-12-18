use std::error::Error;
use std::fmt::Formatter;

use ApplicationException::*;

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

impl std::fmt::Display for ApplicationException {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}
impl std::error::Error for ApplicationException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            InternalException(internal) => Some(internal.as_ref()),
            _ => None,
        }
    }
}

pub type UnknownException = Box<dyn std::error::Error + Send + Sync>;

pub type ApplicationResult<T> = Result<T, ApplicationException>;
pub type UnknownResult<T> = Result<T, UnknownException>;
