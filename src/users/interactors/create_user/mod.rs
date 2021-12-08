use crate::errors::ApplicationResult;
use crate::utils::Interactor;

pub struct CreateUserInteractor {}

impl CreateUserInteractor {
    pub fn new() -> Self {
        CreateUserInteractor {}
    }
}

pub struct CreateUserInput {}

pub struct CreateUserOutput {}

#[async_trait::async_trait]
impl Interactor<CreateUserInput, CreateUserOutput> for CreateUserInteractor {
    async fn execute(&self, input: CreateUserInput) -> ApplicationResult<CreateUserOutput> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_interactor() -> CreateUserInteractor {
        CreateUserInteractor::new()
    }
}