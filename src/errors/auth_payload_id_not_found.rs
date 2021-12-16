use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct AuthPayloadIdNotFound {
    id: String,
}

impl Display for AuthPayloadIdNotFound {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthPayloadIdNotFound {{ id: {} }}", self.id)
    }
}

impl Error for AuthPayloadIdNotFound {}
