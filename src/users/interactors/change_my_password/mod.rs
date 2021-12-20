use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::errors::validation::ValidationError;
use crate::errors::ApplicationResult;
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

#[derive(WithDeps)]
pub struct ChangeMyPasswordInteractor {
    repo: Arc<dyn UsersRepository>,
    crypto: Arc<dyn CryptoService>,
    authorizer: Arc<dyn Authorizer>,
    auth_payload_resolver: Arc<dyn AuthPayloadResolver>,
}

impl ChangeMyPasswordInteractor {
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: ChangeMyPasswordInput,
    ) -> ApplicationResult<()> {
        input.validate()?;
        let mut user = self.auth_payload_resolver.resolve(auth).await?;

        self.authorizer
            .authorize_or_fail(&user, &input.old_password)
            .await?;

        let password = self.crypto.hash(&input.new_password).await?;

        user.password = password;
        self.repo.update(&user).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::auth_payload_resolver_spy::AuthPayloadResolverSpy;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::test_utils::crypto::authorizer_spy::AuthorizerSpy;
    use crate::test_utils::crypto::crypto_service_spy::{CryptoServiceSpy, HASH_RESULT};
    use crate::test_utils::errors_assertion::{
        assert_bad_request_error, assert_validation_error_with_key,
    };
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    #[allow(unused)]
    struct CreationResult {
        interactor: ChangeMyPasswordInteractor,
        repo: Arc<FakeUsersRepository>,
        crypto: Arc<CryptoServiceSpy>,
        authorizer: Arc<AuthorizerSpy>,
        auth_resolver: Arc<AuthPayloadResolverSpy>,
    }
    fn create_interactor() -> CreationResult {
        let repo = Arc::new(FakeUsersRepository::new_with_data(&[User {
            id: auth().get_user_id(),
            email: "".into(),
            password: valid_input().old_password,
            role: Box::from(RoleSpy::new_allowed()),
            name: "".into(),
        }]));
        let crypto = Arc::new(CryptoServiceSpy::new_verified());
        let authorizer = Arc::new(AuthorizerSpy::new_authorized());
        let resolver = Arc::new(AuthPayloadResolverSpy::new_returning(resolved_user()));

        let interactor = ChangeMyPasswordInteractor::new(
            repo.clone(),
            crypto.clone(),
            authorizer.clone(),
            resolver.clone(),
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
            email: "".into(),
            password: "".into(),
            role: Box::from(RoleSpy::new_allowed()),
            name: "".into(),
        }
    }

    #[tokio::test]
    async fn should_return_validation_exception_if_the_new_password_is_not_valid() {
        let c = create_interactor();

        let invalid = ChangeMyPasswordInput {
            old_password: "old_password".into(),
            new_password: "".into(),
        };

        let result = c.interactor.execute(&auth(), invalid).await.unwrap_err();

        assert_validation_error_with_key(result, "new_password");
    }

    #[tokio::test]
    async fn should_throw_if_authorizer_refuses_the_old_password() {
        let mut c = create_interactor();
        c.interactor
            .set_authorizer(Arc::new(AuthorizerSpy::new_unauthorized()).clone());
        let result = c
            .interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();

        assert_bad_request_error(result)
    }

    #[tokio::test]
    async fn should_pass_old_password_with_user_to_authorizer() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        let (authorized_user, authorized_password) = c.authorizer.get_calls()[0].clone();

        assert_eq!(authorized_password, valid_input().old_password);
        assert_eq!(authorized_user.id, resolved_user().id);
    }

    #[tokio::test]
    async fn should_call_crypto_service_for_hashing_new_password() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        c.crypto.assert_hash_calls(&[&valid_input().new_password]);
    }
    #[tokio::test]
    async fn should_store_the_hashed_password_in_repository() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(c.repo.get_users()[0].password, HASH_RESULT);
    }
    fn valid_input() -> ChangeMyPasswordInput {
        ChangeMyPasswordInput {
            old_password: "old_password".into(),
            new_password: "new_password".into(),
        }
    }
    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ALLOWED_ID".into())
    }
}
