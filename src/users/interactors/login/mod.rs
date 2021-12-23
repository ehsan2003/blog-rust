use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use ApplicationException::*;

use crate::access_management::RoleNamer;
use crate::errors::{ApplicationException, ApplicationResult};
use crate::users::interactors::traits::UsersRepository;
use crate::utils::Authorizer;

#[derive(WithDeps)]
pub struct LoginInteractor {
    pub repo: Arc<dyn UsersRepository>,
    pub authorizer: Arc<dyn Authorizer>,
    pub role_namer: Arc<dyn RoleNamer>,
}
#[derive(Debug, Clone)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Clone)]
pub struct LoginOutput {
    user_id: String,
    role: String,
}

const CREDENTIALS_ERROR: &'static str = "invalid credentials";

#[allow(unused)]
impl LoginInteractor {
    pub async fn execute(&self, input: LoginInput) -> ApplicationResult<LoginOutput> {
        let error = BadRequestException("invalid credentials".into());
        let user = self
            .repo
            .get_by_email_or_fail(&input.email)
            .await
            .map_err(|_| BadRequestException(CREDENTIALS_ERROR.into()))?;

        self.authorizer
            .authorize_or_fail(&user, &input.password)
            .await
            .map_err(|_| BadRequestException(CREDENTIALS_ERROR.into()))?;

        Ok(LoginOutput {
            user_id: user.id.into(),
            role: self.role_namer.name_role(user.role),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::role_namer_spy::RoleNamerSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::test_utils::crypto::authorizer_spy::AuthorizerSpy;
    use crate::test_utils::errors_assertion::assert_bad_request_error;
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    fn initial_user() -> User {
        User {
            id: "1".into(),
            email: "a@email.com".into(),
            password: "password".into(),
            role: Box::from(RoleSpy::new_allowed()),
            name: "name".into(),
        }
    }

    fn valid_input() -> LoginInput {
        let initial = initial_user();
        LoginInput {
            email: initial.email.clone(),
            password: initial.password.clone(),
        }
    }

    make_interactor_setup!(
        LoginInteractor,
        [
            (
                repo,
                FakeUsersRepository::new_with_data(&[initial_user()]),
                FakeUsersRepository
            ),
            (authorizer, AuthorizerSpy::new_authorized(), AuthorizerSpy),
            (
                role_namer,
                RoleNamerSpy::new_returning("named_role".into()),
                RoleNamerSpy
            )
        ]
    );
    #[tokio::test]
    async fn should_throw_error_if_user_does_not_exist() {
        let c = create_interactor();
        let input_with_not_existing_email = LoginInput {
            email: "not_found@email.com".into(),
            password: "password".into(),
        };

        let err = c
            .interactor
            .execute(input_with_not_existing_email)
            .await
            .unwrap_err();

        assert_bad_request_error(err);
    }

    #[tokio::test]
    async fn should_throw_bad_request_if_authorizer_refuses_the_password() {
        let mut c = create_interactor();

        c.interactor
            .set_authorizer(Arc::new(AuthorizerSpy::new_unauthorized()));

        let err = c
            .interactor
            .execute(LoginInput {
                email: "a@email.com".into(),
                password: "wrong_password".into(),
            })
            .await
            .unwrap_err();

        assert_bad_request_error(err);
    }
    #[tokio::test]
    async fn should_pass_user_and_role_to_authorizer() {
        let c = create_interactor();

        let input = valid_input();

        c.interactor.execute(input.clone()).await.unwrap();

        let (user, password) = c.authorizer.get_calls()[0].clone();
        assert_eq!(user.email, input.email);
        assert_eq!(password, input.password);
    }
    #[tokio::test]
    async fn should_return_user_id_and_role_name() {
        let mut c = create_interactor();
        c.interactor
            .set_role_namer(Arc::new(RoleNamerSpy::new_returning("role".into())));

        let output = c.interactor.execute(valid_input()).await.unwrap();

        assert_eq!(output.user_id, initial_user().id);
        assert_eq!(output.role, "role");
    }
}
