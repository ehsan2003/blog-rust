use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::errors::ApplicationResult;
use crate::users::interactors::actions::DELETE_USER_ACTION;
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{AuthPayload, AuthRevoker, AuthWithPasswordValidator};

#[derive(WithDeps)]
pub struct DeleteUserInteractor {
    repo: Arc<dyn UsersRepository>,
    auth_with_password_validator: Arc<dyn AuthWithPasswordValidator>,
    revoker: Arc<dyn AuthRevoker>,
}

pub struct DeleteUserInput {
    pub id: String,
    pub password: String,
}

impl DeleteUserInteractor {
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: DeleteUserInput,
    ) -> ApplicationResult<()> {
        auth.can_or_fail(DELETE_USER_ACTION)?;

        self.auth_with_password_validator
            .validate_or_fail(auth, &input.password)
            .await?;
        let user = self.repo.get_by_id_or_fail(&input.id).await?;
        self.revoker.revoke_all_with_id(&user.id).await?;
        self.repo.delete(&user.id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::auth_payload_revoker_spy::AuthRevokerSpy;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::access_management::auth_with_password_validator_spy::AuthWithPasswordValidatorSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::test_utils::errors_assertion::{
        assert_bad_request_error, assert_forbidden_error, assert_not_found_error,
    };
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    fn user() -> User {
        User {
            id: "id".into(),
            email: "email".into(),
            password: "password".into(),
            name: "name".into(),
            role: Box::from(RoleSpy::new_allowed()),
        }
    }

    make_interactor_setup!(
        DeleteUserInteractor,
        [
            (
                repo,
                FakeUsersRepository::new_with_data(&[user()]),
                FakeUsersRepository
            ),
            (
                auth_with_password_validator,
                AuthWithPasswordValidatorSpy::new_verified(),
                AuthWithPasswordValidatorSpy
            ),
            (revoker, AuthRevokerSpy::new(), AuthRevokerSpy)
        ]
    );
    #[tokio::test]
    async fn should_throw_not_found_error_if_user_not_found() {
        let c = create_interactor();
        let input = DeleteUserInput {
            id: "not found id".into(),
            password: "password".into(),
        };

        let result = c.interactor.execute(&auth(), input).await.unwrap_err();

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn should_throw_bad_request_error_if_auth_with_password_validator_is_not_verified() {
        let mut c = create_interactor();
        c.interactor.set_auth_with_password_validator(Arc::new(
            AuthWithPasswordValidatorSpy::new_unverified(),
        ));
        let result = c
            .interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();

        assert_bad_request_error(result);
    }

    #[tokio::test]
    async fn should_pass_auth_payload_and_password_to_auth_with_password_validator() {
        let c = create_interactor();
        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(
            c.auth_with_password_validator.get_called_with(),
            [(auth().get_user_id(), valid_input().password.clone())]
        );
    }

    #[tokio::test]
    async fn should_call_revoker() {
        let c = create_interactor();
        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(c.revoker.get_revoked_ids(), [valid_input().id]);
    }

    #[tokio::test]
    async fn should_throw_forbidden_error_if_the_user_is_not_allowed_to_delete_another_user() {
        let c = create_interactor();
        let auth = &AuthPayloadSpy::new_disallowed("id".to_string());

        let result = c.interactor.execute(auth, valid_input()).await.unwrap_err();

        assert_forbidden_error(result);
    }

    #[tokio::test]
    async fn should_delete_the_user_from_reporsitory() {
        let c = create_interactor();
        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(c.repo.get_users().len(), 0);
    }

    pub fn valid_input() -> DeleteUserInput {
        DeleteUserInput {
            id: user().id,
            password: "password".into(),
        }
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("id".into())
    }
}
