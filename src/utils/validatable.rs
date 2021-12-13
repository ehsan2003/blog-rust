use crate::errors::ValidationError;

pub trait Validatable {
    fn is_valid(&self) -> Result<(), ValidationError>;
}
