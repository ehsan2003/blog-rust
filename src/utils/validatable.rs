use crate::errors::ValidationError;

pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationError>;
}
