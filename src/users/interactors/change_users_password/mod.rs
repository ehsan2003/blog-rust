use std::sync::Arc;

use ApplicationException::*;

use crate::errors::{ApplicationException, ApplicationResult};
use crate::users::interactors::actions::CHANGE_OTHERS_PASSWORD_ACTION;
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{AuthPayload, AuthPayloadResolver, Authorizer, CryptoService};

pub struct ChangeUsersPasswordInteractor {
    repo: Arc<dyn UsersRepository>,
    authorizer: Arc<dyn Authorizer>,
    crypto: Arc<dyn CryptoService>,
    auth_resolver: Arc<dyn AuthPayloadResolver>,
}
#[allow(unused)]
impl ChangeUsersPasswordInteractor {
    pub fn new(
        repo: Arc<dyn UsersRepository>,
        authorizer: Arc<dyn Authorizer>,
        crypto: Arc<dyn CryptoService>,
        auth_resolver: Arc<dyn AuthPayloadResolver>,
    ) -> Self {
        Self {
            repo,
            authorizer,
            crypto,
            auth_resolver,
        }
    }
    pub fn set_repo(&mut self, repo: Arc<dyn UsersRepository>) {
        self.repo = repo;
    }
    pub fn set_authorizer(&mut self, authorizer: Arc<dyn Authorizer>) {
        self.authorizer = authorizer;
    }
    pub fn set_crypto(&mut self, crypto: Arc<dyn CryptoService>) {
        self.crypto = crypto;
    }
    pub fn set_auth_resolver(&mut self, auth_resolver: Arc<dyn AuthPayloadResolver>) {
        self.auth_resolver = auth_resolver;
    }
}
pub struct ChangeUsersPasswordInput {
    pub user_id: String,
    pub new_password: String,
    pub password: String,
}
impl ChangeUsersPasswordInteractor {
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: ChangeUsersPasswordInput,
    ) -> ApplicationResult<()> {
        if !auth.can(CHANGE_OTHERS_PASSWORD_ACTION) {
            return Err(ForBiddenException("".into()));
        }
        let modifier_user = self.auth_resolver.resolve(auth).await?;
        if !self
            .authorizer
            .authorize(&modifier_user, &input.password)
            .await?
        {
            return Err(ForBiddenException("".into()));
        }

        let mut user = match self.repo.get_by_id(&input.user_id).await? {
            None => return Err(NotFoundException("".into())),
            Some(user) => user,
        };
        user.password = self.crypto.hash(&input.new_password).await?;
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
    use crate::test_utils::errors_assertion::{assert_forbidden_error, assert_not_found_error};
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    struct CreationResult {
        interactor: ChangeUsersPasswordInteractor,
        repo: Arc<FakeUsersRepository>,
        authorizer: Arc<AuthorizerSpy>,
        crypto: Arc<CryptoServiceSpy>,
        resolver: Arc<AuthPayloadResolverSpy>,
    }
    fn modifying_user() -> User {
        User {
            id: "1".into(),
            email: "a@email.com".into(),
            password: "password".into(),
            name: "modifying".into(),
            role: Box::from(RoleSpy::new_allowed()),
        }
    }
    fn modifier_user() -> User {
        User {
            id: "2".into(),
            email: "b@email.com".into(),
            password: "password".into(),
            name: "modifier".into(),
            role: Box::from(RoleSpy::new_allowed()),
        }
    }
    fn valid_input() -> ChangeUsersPasswordInput {
        let initial_user = modifying_user();

        ChangeUsersPasswordInput {
            user_id: initial_user.id,
            new_password: "new_password".into(),
            password: "password".into(),
        }
    }
    fn create_interactor() -> CreationResult {
        let repo = Arc::new(FakeUsersRepository::new_with_data(&[
            modifying_user(),
            modifier_user(),
        ]));
        let authorizer = Arc::new(AuthorizerSpy::new_authorized());
        let crypto = Arc::new(CryptoServiceSpy::new_verified());
        let resolver = Arc::new(AuthPayloadResolverSpy::new_returning(modifier_user()));
        let interactor = ChangeUsersPasswordInteractor::new(
            repo.clone(),
            authorizer.clone(),
            crypto.clone(),
            resolver.clone(),
        );
        CreationResult {
            interactor,
            repo,
            authorizer,
            crypto,
            resolver,
        }
    }

    #[tokio::test]
    async fn should_throw_error_if_user_id_not_found() {
        let CreationResult { interactor, .. } = create_interactor();
        let mut valid_input = valid_input();
        valid_input.user_id = "not found".into();
        let error = interactor.execute(&auth(), valid_input).await.unwrap_err();
        assert_not_found_error(error);
    }

    #[tokio::test]
    async fn should_throw_error_if_payload_is_not_allowed() {
        let CreationResult { interactor, .. } = create_interactor();
        let payload_spy = AuthPayloadSpy::new_disallowed("WEAK".into());
        let valid_input = valid_input();
        let error = interactor
            .execute(&payload_spy, valid_input)
            .await
            .unwrap_err();
        assert_forbidden_error(error);
    }

    #[tokio::test]
    async fn should_pass_appropriate_action_to_payload() {
        let CreationResult { interactor, .. } = create_interactor();
        let payload_spy = auth();
        let valid_input = valid_input();
        interactor.execute(&payload_spy, valid_input).await.unwrap();
        assert_eq!(payload_spy.get_called(), [CHANGE_OTHERS_PASSWORD_ACTION]);
    }

    #[tokio::test]
    async fn should_throw_forbidden_error_if_password_is_not_correct() {
        let CreationResult { mut interactor, .. } = create_interactor();
        let authorizer = Arc::new(AuthorizerSpy::new_unauthorized());
        interactor.set_authorizer(authorizer);
        let error = interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();
        assert_forbidden_error(error);
    }

    #[tokio::test]
    async fn should_resolve_auth_payload() {
        let CreationResult {
            interactor,
            resolver,
            ..
        } = create_interactor();

        let payload_spy = &auth();
        interactor
            .execute(payload_spy, valid_input())
            .await
            .unwrap();
        assert_eq!(
            *resolver.payload_ids.lock().unwrap(),
            [payload_spy.get_user_id()]
        );
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ALLOWED_ID".into())
    }

    #[tokio::test]
    async fn should_pass_new_password_to_hash() {
        let CreationResult {
            interactor, crypto, ..
        } = create_interactor();

        interactor.execute(&auth(), valid_input()).await.unwrap();

        crypto.assert_hash_calls(&[&valid_input().new_password]);
    }

    #[tokio::test]
    async fn should_store_password_in_repository() {
        let CreationResult {
            interactor, repo, ..
        } = create_interactor();

        interactor.execute(&auth(), valid_input()).await.unwrap();

        let user = repo.get_by_id(&modifying_user().id).await.unwrap().unwrap();
        assert_eq!(user.password, HASH_RESULT);
    }
}
