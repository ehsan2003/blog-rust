use std::sync::Arc;

use crate::access_management::RoleFactory;
use crate::errors::{ApplicationException, ApplicationResult, ValidationError};
use crate::users::domain::User;
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{CryptoService, Interactor, RandomService, Validatable};

pub struct CreateUserInteractor {
    random_service: Arc<dyn RandomService>,
    crypto_service: Arc<dyn CryptoService>,
    repo: Arc<dyn UsersRepository>,
    role_factory: Arc<dyn RoleFactory>,
}

// dependencies
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

    fn validate_or_fail(&self, input: &CreateUserInput) -> ApplicationResult<()> {
        input.validate()?;

        if !self.role_factory.is_valid_role_name(&input.role) {
            return Err(ApplicationException::ValidationException {
                key: "role".to_owned(),
                value: input.role.clone(),
                message: format!("Role {} not found", input.role),
            });
        }
        Ok(())
    }

    async fn check_email_or_fail(&self, input: &CreateUserInput) -> ApplicationResult<()> {
        if self.repo.email_exists(&input.email).await? {
            return Err(ApplicationException::DuplicationException {
                key: "email".to_string(),
                value: input.email.clone(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CreateUserInput {
    role: String,
    email: String,
    name: String,
}

impl Validatable for CreateUserInput {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.role.is_empty() {
            return Err(ValidationError::new(
                "role".to_string(),
                self.role.clone(),
                "role is required".to_string(),
            ));
        }
        if self.email.is_empty() {
            return Err(ValidationError::new(
                "email".to_string(),
                self.email.clone(),
                "email is required".to_string(),
            ));
        }
        if !validator::validate_email(&self.email) {
            println!("{}", self.email);
            return Err(ValidationError::new(
                "email".to_string(),
                self.email.clone(),
                "email is invalid".to_string(),
            ));
        }
        if self.name.is_empty() {
            return Err(ValidationError::new(
                "name".to_string(),
                self.name.clone(),
                "name is required".to_string(),
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CreateUserOutput {
    password: String,
    user_id: String,
}

#[async_trait::async_trait]
impl Interactor<CreateUserInput, CreateUserOutput> for CreateUserInteractor {
    async fn execute(&self, input: CreateUserInput) -> ApplicationResult<CreateUserOutput> {
        self.validate_or_fail(&input)?;
        self.check_email_or_fail(&input).await?;
        let random_password = self.random_service.secure_random_password().await?;
        let password_hash = self.crypto_service.hash(&random_password).await?;

        let role = self.role_factory.create_role(&input.role).unwrap();

        self.repo
            .create(&User {
                email: input.email.clone(),
                name: input.name.clone(),
                password: password_hash,
                role,
                id: self.random_service.random_id().await?,
            })
            .await?;
        Ok(CreateUserOutput {
            password: random_password,
            user_id: input.email.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::allowed_role::AllowedRole;
    use crate::test_utils::access_management::allowed_role_role_factory_spy::AllowedRoleRoleFactorySpy;
    use crate::test_utils::access_management::unknown_role_role_factory_spy::UnknownRoleRoleFactorySpy;
    use crate::test_utils::crypto::dummy_crypto_service::{CryptoServiceSpy, HASH_RESULT};
    use crate::test_utils::crypto::dummy_random_service::{
        RandomServiceSpy, RANDOM_ID, SECURE_RANDOM_PASSWORD,
    };
    use crate::users::domain::User;
    use crate::users::interactors::mocks::dummy_users_repository::FakeUsersRepository;

    use super::*;

    fn create_interactor() -> (
        CreateUserInteractor,
        Arc<FakeUsersRepository>,
        Arc<AllowedRoleRoleFactorySpy>,
        Arc<CryptoServiceSpy>,
        Arc<RandomServiceSpy>,
    ) {
        let random_service = Arc::new(RandomServiceSpy::new());
        let crypto_service = Arc::new(CryptoServiceSpy::new_verified());
        let repo = Arc::new(FakeUsersRepository::new_empty());
        let role_factory = Arc::new(AllowedRoleRoleFactorySpy::new());

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
        let inputs = vec![CreateUserInput {
            role: "test".to_owned(),
            name: "pest".to_owned(),
            email: "b.com".to_owned(),
        }];
        for input in inputs {
            let error = i.execute(input).await.unwrap_err();
            assert_validation_error(error);
        }
    }

    #[tokio::test]
    async fn should_throw_validation_error_when_the_role_is_unknown_for_the_role_factory() {
        let (mut i, ..) = create_interactor();
        i.set_role_factory(Arc::new(UnknownRoleRoleFactorySpy::new()));
        let error = i.execute(valid_input()).await.unwrap_err();
        assert_validation_error_with_key(error, "role");
    }

    #[tokio::test]
    async fn should_call_role_factory_with_passed_role() {
        let (i, _, spy, ..) = create_interactor();

        let valid_input = valid_input();

        i.execute(valid_input.clone()).await.unwrap();
        let called_with = spy.called_with.lock().unwrap();
        assert_eq!(*called_with, vec![valid_input.role.clone()]);
    }
    #[tokio::test]
    async fn should_call_generate_random_password() {
        let (i, _, _, _, spy) = create_interactor();
        let valid_input = valid_input();
        i.execute(valid_input.clone()).await.unwrap();

        spy.assert_secure_random_called();
    }
    #[tokio::test]
    async fn should_hash_generated_password() {
        let (i, _, _, crypto_service, _) = create_interactor();
        i.execute(valid_input()).await.unwrap();
        crypto_service.assert_hash_calls(vec![SECURE_RANDOM_PASSWORD.to_owned()]);
    }
    #[tokio::test]
    async fn should_throw_duplication_exception_when_email_already_exists() {
        let (mut i, _, _, _, _) = create_interactor();
        let input = valid_input();

        i.set_repo(Arc::new(FakeUsersRepository::new_with_data(vec![User {
            email: input.email,
            name: input.name,
            role: Box::from(AllowedRole {}),
            password: "exists".to_owned(),
            id: "id".to_owned(),
        }])));

        let err = i.execute(valid_input()).await.unwrap_err();
        assert_duplication_error(err, "email")
    }
    #[tokio::test]
    async fn should_store_a_user_in_repo_which_contains_correct_fields() {
        let (i, repo, _, _, _) = create_interactor();
        let input = valid_input();
        i.execute(input.clone()).await.unwrap();
        let stored_user = repo
            .users
            .lock()
            .unwrap()
            .get(0)
            .expect("should have a user")
            .clone();
        assert_eq!(stored_user.email, input.email);
        assert_eq!(stored_user.name, input.name);
        assert_eq!(stored_user.password, HASH_RESULT);
        assert_eq!(stored_user.id, RANDOM_ID);
    }
    #[tokio::test]
    async fn should_return_random_password_on_result() {
        let (i, _, _, _, _) = create_interactor();
        let result = i.execute(valid_input()).await.unwrap();
        assert_eq!(result.password, SECURE_RANDOM_PASSWORD);
    }

    fn assert_duplication_error(err: ApplicationException, valid_key: &str) {
        if let ApplicationException::DuplicationException { key, .. } = err {
            assert_eq!(key, valid_key);
        } else {
            assert!(false);
        }
    }

    fn valid_input() -> CreateUserInput {
        CreateUserInput {
            role: "test".to_owned(),
            name: "pest".to_owned(),
            email: "t@email.com".to_owned(),
        }
    }
    async fn assert_validation_error(error: ApplicationException) {
        if let ApplicationException::ValidationException { .. } = error {
        } else {
            assert!(false);
        }
    }
    async fn assert_validation_error_with_key(error: ApplicationException, valid_key: &str) {
        if let ApplicationException::ValidationException { key, .. } = error {
            assert_eq!(valid_key, key);
        } else {
            assert!(false, "expected ValidationException");
        }
    }
}
