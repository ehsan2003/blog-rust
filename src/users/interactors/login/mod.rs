use std::sync::Arc;

use ApplicationException::*;

use crate::access_management::RoleNamer;
use crate::errors::{ApplicationException, ApplicationResult};
use crate::users::interactors::traits::UsersRepository;
use crate::utils::Authorizer;

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
#[allow(unused)]
impl LoginInteractor {
    pub fn new(
        repo: Arc<dyn UsersRepository>,
        authorizer: Arc<dyn Authorizer>,
        role_namer: Arc<dyn RoleNamer>,
    ) -> Self {
        Self {
            repo,
            authorizer,
            role_namer,
        }
    }
    pub fn set_repo(&mut self, repo: Arc<dyn UsersRepository>) {
        self.repo = repo;
    }
    pub fn set_authorizer(&mut self, authorizer: Arc<dyn Authorizer>) {
        self.authorizer = authorizer;
    }
    pub fn set_role_namer(&mut self, role_namer: Arc<dyn RoleNamer>) {
        self.role_namer = role_namer;
    }

    pub async fn execute(&self, input: LoginInput) -> ApplicationResult<LoginOutput> {
        let user = match self.repo.get_by_email(&input.email).await? {
            None => return Err(BadRequestException("invalid credentials".into())),
            Some(user) => user,
        };
        let is_correct = self.authorizer.authorize(&user, &input.password).await?;
        if !is_correct {
            return Err(BadRequestException("invalid credentials".into()));
        }
        Ok(LoginOutput {
            user_id: user.id.into(),
            role: self.role_namer.name_role(user.role).unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::role_namer_spy::RoleNamerSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::test_utils::crypto::authorizer_spy::AuthorizerSpy;
    use crate::test_utils::errors_assertion::assert_bad_request_error;
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    #[allow(unused)]
    struct CreationResult {
        interactor: LoginInteractor,
        repo: Arc<FakeUsersRepository>,
        authorizer: Arc<AuthorizerSpy>,
        role_namer: Arc<RoleNamerSpy>,
    }
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
    fn create_interactor() -> CreationResult {
        let repo = Arc::new(FakeUsersRepository::new_with_data(vec![initial_user()]));
        let authorizer = Arc::new(AuthorizerSpy::new_authorized());
        let role_namer = Arc::new(RoleNamerSpy::new_returning("named_role".into()));

        let repo_clone = repo.clone();
        let authorizer_clone = authorizer.clone();
        let role_namer_clone = role_namer.clone();

        let interactor = LoginInteractor::new(repo_clone, authorizer_clone, role_namer_clone);

        CreationResult {
            interactor,
            repo,
            authorizer,
            role_namer,
        }
    }

    #[tokio::test]
    async fn should_throw_error_if_user_does_not_exist() {
        let CreationResult { interactor, .. } = create_interactor();

        let input = LoginInput {
            email: "not_found@email.com".into(),
            password: "password".into(),
        };
        let err = interactor.execute(input).await.unwrap_err();
        assert_bad_request_error(err);
    }

    #[tokio::test]
    async fn should_throw_bad_request_if_authorizer_refuses_the_password() {
        let CreationResult { mut interactor, .. } = create_interactor();
        interactor.set_authorizer(Arc::new(AuthorizerSpy::new_unauthorized()));
        let input = LoginInput {
            email: "a@email.com".into(),
            password: "wrong_password".into(),
        };
        let err = interactor.execute(input).await.unwrap_err();
        assert_bad_request_error(err);
    }
    #[tokio::test]
    async fn should_pass_user_and_role_to_authorizer() {
        let CreationResult {
            interactor,
            authorizer,
            ..
        } = create_interactor();
        let input = valid_input();
        interactor.execute(input.clone()).await.unwrap();
        let a = authorizer.get_calls()[0].clone();
        let user = a.0.clone();
        let password = a.1.clone();
        assert_eq!(user.email, input.email);
        assert_eq!(password, input.password);
    }
    #[tokio::test]
    async fn should_return_user_id_and_role_name() {
        let CreationResult { mut interactor, .. } = create_interactor();
        let input = valid_input();
        interactor.set_role_namer(Arc::new(RoleNamerSpy::new_returning("role".into())));
        let output = interactor.execute(input).await.unwrap();
        let initial_user = initial_user();
        assert_eq!(output.user_id, initial_user.id);
        assert_eq!(output.role, "role");
    }
}
