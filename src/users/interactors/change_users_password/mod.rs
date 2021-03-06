use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::errors::ApplicationResult;
use crate::users::interactors::actions::CHANGE_OTHERS_PASSWORD_ACTION;
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{AuthPayload, AuthWithPasswordValidator, CryptoService};

#[derive(WithDeps)]
pub struct ChangeUsersPasswordInteractor {
    repo: Arc<dyn UsersRepository>,
    crypto: Arc<dyn CryptoService>,
    auth_with_password_validator: Arc<dyn AuthWithPasswordValidator>,
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
        auth.can_or_fail(CHANGE_OTHERS_PASSWORD_ACTION)?;

        self.auth_with_password_validator
            .validate_or_fail(auth, &input.password)
            .await?;

        let mut user = self.repo.get_by_id_or_fail(&input.user_id).await?;
        user.password = self.crypto.hash(&input.new_password).await?;
        self.repo.update(&user).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::access_management::auth_with_password_validator_spy::AuthWithPasswordValidatorSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::test_utils::crypto::crypto_service_spy::{CryptoServiceSpy, HASH_RESULT};
    use crate::test_utils::errors_assertion::{
        assert_bad_request_error, assert_forbidden_error, assert_not_found_error,
    };
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

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

    make_interactor_setup!(
        ChangeUsersPasswordInteractor,
        [
            (
                repo,
                FakeUsersRepository::new_with_data(&[modifying_user(), modifier_user(),]),
                FakeUsersRepository
            ),
            (crypto, CryptoServiceSpy::new_verified(), CryptoServiceSpy),
            (
                auth_with_password_validator,
                AuthWithPasswordValidatorSpy::new_verified(),
                AuthWithPasswordValidatorSpy
            )
        ]
    );
    #[tokio::test]
    async fn should_throw_error_if_user_id_not_found() {
        let c = create_interactor();

        let mut valid_input = valid_input();
        valid_input.user_id = "not found".into();

        let error = c
            .interactor
            .execute(&auth(), valid_input)
            .await
            .unwrap_err();

        assert_not_found_error(error);
    }

    #[tokio::test]
    async fn should_throw_error_if_payload_is_not_allowed() {
        let c = create_interactor();
        let payload_spy = AuthPayloadSpy::new_disallowed("WEAK".into());
        let valid_input = valid_input();

        let error = c
            .interactor
            .execute(&payload_spy, valid_input)
            .await
            .unwrap_err();

        assert_forbidden_error(error);
    }

    #[tokio::test]
    async fn should_pass_appropriate_action_to_payload() {
        let c = create_interactor();
        let payload_spy = auth();
        let valid_input = valid_input();

        c.interactor
            .execute(&payload_spy, valid_input)
            .await
            .unwrap();

        assert_eq!(payload_spy.get_called(), [CHANGE_OTHERS_PASSWORD_ACTION]);
    }

    #[tokio::test]
    async fn should_throw_bad_request_error_if_password_is_not_correct() {
        let mut c = create_interactor();
        c.interactor.set_auth_with_password_validator(Arc::from(
            AuthWithPasswordValidatorSpy::new_unverified(),
        ));
        let error = c
            .interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();

        assert_bad_request_error(error);
    }

    #[tokio::test]
    async fn should_pass_auth_payload_and_password_to_auth_with_password_validator() {
        let c = create_interactor();

        let payload_spy = &auth();

        c.interactor
            .execute(payload_spy, valid_input())
            .await
            .unwrap();

        assert_eq!(
            *c.auth_with_password_validator.get_called_with(),
            [(payload_spy.get_user_id(), valid_input().password.into())]
        );
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ALLOWED_ID".into())
    }

    #[tokio::test]
    async fn should_pass_new_password_to_hash() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        c.crypto.assert_hash_calls(&[&valid_input().new_password]);
    }

    #[tokio::test]
    async fn should_store_password_in_repository() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        let user = c
            .repo
            .get_by_id(&modifying_user().id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.password, HASH_RESULT);
    }
}
