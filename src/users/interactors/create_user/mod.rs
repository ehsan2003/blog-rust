use std::sync::Arc;

use crate::access_management::RoleFactory;
use crate::errors::{ApplicationException, ApplicationResult};
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{CryptoService, Interactor, RandomService, Validatable};

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

#[derive(Debug, Clone)]
pub struct CreateUserInput {
    user_role: String,
    user_email: String,
    user_name: String,
}

impl Validatable for CreateUserInput {
    fn is_valid(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct CreateUserOutput {
    password: String,
    user_id: String,
}

#[async_trait::async_trait]
impl Interactor<CreateUserInput, CreateUserOutput> for CreateUserInteractor {
    async fn execute(&self, _input: CreateUserInput) -> ApplicationResult<CreateUserOutput> {
        Err(ApplicationException::ValidationException { key: "".to_string(), message: "".to_string(), value: "".to_string() })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::role_factory_spy::RoleFactorySpy;
    use crate::test_utils::crypto::dummy_crypto_service::DummyCryptoService;
    use crate::test_utils::crypto::dummy_random_service::DummyRandomService;
    use crate::users::interactors::mocks::dummy_users_repository::DummyUsersRepository;

    use super::*;

    struct CreationResult {}

    fn create_interactor() -> (CreateUserInteractor,
                               Arc<DummyUsersRepository>,
                               Arc<RoleFactorySpy>,
                               Arc<DummyCryptoService>,
                               Arc<DummyRandomService>, ) {
        let random_service = Arc::new(DummyRandomService);
        let crypto_service = Arc::new(DummyCryptoService);
        let repo = Arc::new(DummyUsersRepository);
        let role_factory = Arc::new(RoleFactorySpy::new());

        let arc_cloned_random_service = Arc::clone(&random_service);
        let arc_cloned_crypto_service = Arc::clone(&crypto_service);
        let arc_cloned_repo = Arc::clone(&repo);
        let arc_cloned_role_factory = Arc::clone(&role_factory);

        let interactor = CreateUserInteractor::new(
            arc_cloned_random_service,
            arc_cloned_crypto_service,
            arc_cloned_repo,
            arc_cloned_role_factory,
        );

        (
            interactor,
            repo,
            role_factory,
            crypto_service,
            random_service,
        )
    }

    #[tokio::test]
    async fn should_throw_validation_error_when_data_is_invalid() {
        let (i, ..) = create_interactor();
        let data = CreateUserInput {
            user_role: "test".to_owned(),
            user_name: "pest".to_owned(),
            user_email: "a@b.com".to_owned(),
        };
        let _err = i.execute(data).await.unwrap_err();
    }
}