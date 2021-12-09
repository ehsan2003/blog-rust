use std::sync::Arc;

use crate::access_management::RoleFactory;
use crate::errors::ApplicationResult;
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{CryptoService, Interactor, RandomService};

pub struct CreateUserInteractor {
    random_service: Arc<dyn RandomService>,
    crypto_service: Arc<dyn CryptoService>,
    repo: Arc<dyn UsersRepository>,
    role_factory: Arc<dyn RoleFactory>,
}

impl CreateUserInteractor {
    pub fn new(
        random_service: Arc<dyn RandomService>,
        crypto_service: Arc<dyn CryptoService>,
        repo: Arc<dyn UsersRepository>,
        role_factory: Arc<dyn RoleFactory>,
    ) -> Self {
        CreateUserInteractor {
            random_service,
            crypto_service,
            repo,
            role_factory,
        }
    }
    pub fn set_random_service(&mut self, s: Arc<dyn RandomService>) {
        self.random_service = s;
    }
    pub fn set_crypto_service(&mut self, s: Arc<dyn CryptoService>) {
        self.crypto_service = s;
    }
    pub fn set_repo(&mut self, r: Arc<dyn UsersRepository>) {
        self.repo = r;
    }
    pub fn set_role_factory(&mut self, r: Arc<dyn RoleFactory>) {
        self.role_factory = r;
    }
}

pub struct CreateUserInput {
    user_role: String,
    user_email: String,
    user_name: String,
}

pub struct CreateUserOutput {
    password: String,
    user_id: String,
}

#[async_trait::async_trait]
impl Interactor<CreateUserInput, CreateUserOutput> for CreateUserInteractor {
    async fn execute(&self, input: CreateUserInput) -> ApplicationResult<CreateUserOutput> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// fn create_interactor() -> CreateUserInteractor {
    //     CreateUserInteractor::new()
    // }
}