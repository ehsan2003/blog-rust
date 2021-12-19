use std::sync::Arc;

use crate::errors::ApplicationResult;
use crate::users::interactors::actions::DELETE_USER_ACTION;
use crate::users::interactors::traits::UsersRepository;
use crate::utils::{AuthPayload, AuthRevoker, AuthWithPasswordValidator};

pub struct DeleteUserInteractor {
    repo: Arc<dyn UsersRepository>,
    auth_with_password_validator: Arc<dyn AuthWithPasswordValidator>,
    revoker: Arc<dyn AuthRevoker>,
}

#[allow(unused)]
impl DeleteUserInteractor {
    pub fn new(
        repo: Arc<dyn UsersRepository>,
        revoker: Arc<dyn AuthRevoker>,
        auth_with_password_validator: Arc<dyn AuthWithPasswordValidator>,
    ) -> Self {
        Self {
            repo,
            revoker,
            auth_with_password_validator,
        }
    }
    pub fn set_repo(&mut self, repo: Arc<dyn UsersRepository>) {
        self.repo = repo;
    }
    pub fn set_revoker(&mut self, revoker: Arc<dyn AuthRevoker>) {
        self.revoker = revoker;
    }
    pub fn set_auth_with_password_validator(
        &mut self,
        auth_with_password_validator: Arc<dyn AuthWithPasswordValidator>,
    ) {
        self.auth_with_password_validator = auth_with_password_validator;
    }
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

    struct CreationResult {
        interactor: DeleteUserInteractor,
        repo: Arc<FakeUsersRepository>,
        auth_with_password_validator: Arc<AuthWithPasswordValidatorSpy>,
        revoker: Arc<AuthRevokerSpy>,
    }

    fn user() -> User {
        User {
            id: "id".into(),
            email: "email".into(),
            password: "password".into(),
            name: "name".into(),
            role: Box::from(RoleSpy::new_allowed()),
        }
    }

    fn create_interactor() -> CreationResult {
        let auth_with_password_validator = Arc::new(AuthWithPasswordValidatorSpy::new_verified());
        let repo = Arc::new(FakeUsersRepository::new_with_data(&[user()]));
        let revoker = Arc::new(AuthRevokerSpy::new());
        let interactor = DeleteUserInteractor::new(
            repo.clone(),
            revoker.clone(),
            auth_with_password_validator.clone(),
        );
        CreationResult {
            interactor,
            repo,
            auth_with_password_validator,
            revoker,
        }
    }

    #[tokio::test]
    async fn should_throw_not_found_error_if_user_not_found() {
        let CreationResult { interactor: i, .. } = create_interactor();
        let input = DeleteUserInput {
            id: "not found id".into(),
            password: "password".into(),
        };

        let result = i.execute(&auth(), input).await.unwrap_err();

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn should_throw_bad_request_error_if_auth_with_password_validator_is_not_verified() {
        let CreationResult {
            interactor: mut i, ..
        } = create_interactor();

        i.set_auth_with_password_validator(
            Arc::new(AuthWithPasswordValidatorSpy::new_unverified()),
        );
        let result = i.execute(&auth(), valid_input()).await.unwrap_err();

        assert_bad_request_error(result);
    }

    #[tokio::test]
    async fn should_pass_auth_payload_and_password_to_auth_with_password_validator() {
        let CreationResult {
            interactor: i,
            auth_with_password_validator,
            ..
        } = create_interactor();

        i.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(
            auth_with_password_validator.get_called_with(),
            [(auth().get_user_id(), valid_input().password.clone())]
        );
    }

    #[tokio::test]
    async fn should_call_revoker() {
        let CreationResult {
            interactor: i,
            revoker,
            ..
        } = create_interactor();

        i.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(revoker.get_revoked_ids(), [valid_input().id]);
    }

    #[tokio::test]
    async fn should_throw_forbidden_error_if_the_user_is_not_allowed_to_delete_another_user() {
        let CreationResult { interactor: i, .. } = create_interactor();

        let result = i
            .execute(
                &AuthPayloadSpy::new_disallowed("id".to_string()),
                valid_input(),
            )
            .await
            .unwrap_err();

        assert_forbidden_error(result);
    }

    #[tokio::test]
    async fn should_delete_the_user_from_reporsitory() {
        let CreationResult {
            interactor: i,
            repo,
            ..
        } = create_interactor();

        i.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(repo.get_users().len(), 0);
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
