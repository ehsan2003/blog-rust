use crate::errors::validation::ValidationError;

pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationError>;
}
