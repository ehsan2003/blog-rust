use std::sync::Arc;

use crate::errors::validation::ValidationError;
use crate::errors::{ApplicationException, ApplicationResult};
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{AuthPayload, AuthPayloadResolver, Authorizer, CryptoService, Validatable};

pub struct ChangeMyPasswordInput {
    pub old_password: String,
    pub new_password: String,
}
impl Validatable for ChangeMyPasswordInput {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.new_password.is_empty() {
            return Err(ValidationError::new(
                "new_password".into(),
                "*****".into(),
                "".into(),
            ));
        }
        Ok(())
    }
}
pub struct ChangeMyPasswordInteractor {
    repo: Arc<dyn UsersRepository>,
    crypto: Arc<dyn CryptoService>,
    authorizer: Arc<dyn Authorizer>,
    auth_payload_resolver: Arc<dyn AuthPayloadResolver>,
}

impl ChangeMyPasswordInteractor {
    pub fn new(
        repo: Arc<dyn UsersRepository>,
        crypto: Arc<dyn CryptoService>,
        authorizer: Arc<dyn Authorizer>,
        resolver: Arc<dyn AuthPayloadResolver>,
    ) -> Self {
        Self {
            repo,
            crypto,
            authorizer,
            auth_payload_resolver: resolver,
        }
    }
    pub fn set_repo(&mut self, repo: Arc<dyn UsersRepository>) {
        self.repo = repo;
    }
    pub fn set_crypto(&mut self, crypto: Arc<dyn CryptoService>) {
        self.crypto = crypto;
    }
    pub fn set_authorizer(&mut self, authorizer: Arc<dyn Authorizer>) {
        self.authorizer = authorizer;
    }
    pub fn set_auth_payload_resolver(&mut self, r: Arc<dyn AuthPayloadResolver>) {
        self.auth_payload_resolver = r;
    }
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: ChangeMyPasswordInput,
    ) -> ApplicationResult<()> {
        input.validate()?;
        let mut user = self.auth_payload_resolver.resolve(auth).await?;

        if !self
            .authorizer
            .authorize(&user, &input.old_password)
            .await?
        {
            return Err(ApplicationException::ForBiddenException(
                "Old password is wrong".to_string(),
            ));
        }
        let password = self.crypto.hash(&input.new_password).await?;

        user.password = password;
        self.repo.update(&user).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::allowed_auth_payload_spy::AllowedAuthPayloadSpy;
    use crate::test_utils::access_management::allowed_role::AllowedRole;
    use crate::test_utils::access_management::auth_payload_resolver_spy::AuthPayloadResolverSpy;
    use crate::test_utils::crypto::authorizer_spy::AuthorizerSpy;
    use crate::test_utils::crypto::crypto_service_spy::{CryptoServiceSpy, HASH_RESULT};
    use crate::test_utils::errors_assertion::{
        assert_forbidden_error, assert_validation_error_with_key,
    };
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    struct CreationResult {
        interactor: ChangeMyPasswordInteractor,
        repo: Arc<FakeUsersRepository>,
        crypto: Arc<CryptoServiceSpy>,
        authorizer: Arc<AuthorizerSpy>,
        auth_resolver: Arc<AuthPayloadResolverSpy>,
    }
    fn create_interactor() -> CreationResult {
        let repo = Arc::new(FakeUsersRepository::new_with_data(vec![User {
            id: auth().get_user_id(),
            email: "".to_string(),
            password: valid_input().old_password,
            role: Box::from(AllowedRole),
            name: "".to_string(),
        }]));
        let crypto = Arc::new(CryptoServiceSpy::new_verified());
        let authorizer = Arc::new(AuthorizerSpy::new_authorized());
        let resolver = Arc::new(AuthPayloadResolverSpy::new_returning(resolved_user()));
        let repo_clone = repo.clone();
        let crypto_clone = crypto.clone();
        let authorizer_clone = authorizer.clone();
        let resolver_clone = resolver.clone();

        let interactor = ChangeMyPasswordInteractor::new(
            repo_clone,
            crypto_clone,
            authorizer_clone,
            resolver_clone,
        );

        return CreationResult {
            interactor,
            repo,
            crypto,
            authorizer,
            auth_resolver: resolver,
        };
    }

    fn resolved_user() -> User {
        User {
            id: auth().get_user_id(),
            email: "".to_string(),
            password: "".to_string(),
            role: Box::from(AllowedRole),
            name: "".to_string(),
        }
    }

    #[tokio::test]
    async fn should_return_validation_exception_if_the_new_password_is_not_valid() {
        let CreationResult {
            interactor: mut i, ..
        } = create_interactor();

        let invalid = ChangeMyPasswordInput {
            old_password: "old_password".into(),
            new_password: "".into(),
        };

        let result = i.execute(&auth(), invalid).await.unwrap_err();

        assert_validation_error_with_key(result, "new_password");
    }
    #[tokio::test]
    async fn should_throw_if_authorizer_refuses_the_old_password_and_pass_old_password_with_id_to_authorizer(
    ) {
        let CreationResult {
            interactor: mut i, ..
        } = create_interactor();
        let a = Arc::new(AuthorizerSpy::new_unauthorized());
        i.set_authorizer(a.clone());
        let result = i.execute(&auth(), valid_input()).await.unwrap_err();
        let authorizer_calls = a.get_calls().get(0).unwrap().clone();
        assert_eq!(authorizer_calls.1, valid_input().old_password);
        assert_eq!(authorizer_calls.0.id, resolved_user().id);
        assert_forbidden_error(result)
    }

    #[tokio::test]
    async fn should_call_crypto_service_for_hashing_new_password() {
        let CreationResult {
            interactor: i,
            crypto: c,
            ..
        } = create_interactor();

        i.execute(&auth(), valid_input()).await.unwrap();

        c.assert_hash_calls(vec![valid_input().new_password.to_string()]);
    }
    #[tokio::test]
    async fn should_store_the_hashed_password_in_repository() {
        let CreationResult {
            interactor: i,
            repo: r,
            ..
        } = create_interactor();

        i.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(r.users.lock().unwrap()[0].password, HASH_RESULT);
    }
    fn valid_input() -> ChangeMyPasswordInput {
        ChangeMyPasswordInput {
            old_password: "old_password".to_string(),
            new_password: "new_password".to_string(),
        }
    }
    fn auth() -> AllowedAuthPayloadSpy {
        AllowedAuthPayloadSpy::new()
    }
}
