use std::sync::Arc;

use crate::access_management::RoleNamer;
use crate::errors::ApplicationException::ForBiddenException;
use crate::errors::ApplicationResult;
use crate::users::interactors::actions::LIST_USERS_ACTION;
use crate::users::interactors::traits::UsersRepository;
use crate::users::interactors::utils::{get_visible_user, VisibleUser};
use crate::utils::AuthPayload;

#[derive(Debug, Clone)]
pub struct ListUsersOutput {
    pub users: Vec<VisibleUser>,
}
pub struct ListUsersInteractor {
    repo: Arc<dyn UsersRepository>,
    role_namer: Arc<dyn RoleNamer>,
}
#[allow(unused)]
impl ListUsersInteractor {
    pub fn new(repo: Arc<dyn UsersRepository>, role_namer: Arc<dyn RoleNamer>) -> Self {
        Self { repo, role_namer }
    }
    pub fn set_repo(&mut self, repo: Arc<dyn UsersRepository>) {
        self.repo = repo;
    }
    pub fn set_namer(&mut self, namer: Arc<dyn RoleNamer>) {
        self.role_namer = namer;
    }
}

impl ListUsersInteractor {
    pub async fn execute(&self, auth: &(dyn AuthPayload)) -> ApplicationResult<ListUsersOutput> {
        auth.can_or_fail(LIST_USERS_ACTION)?;
        let users = self.repo.get_all().await?;
        return Ok(ListUsersOutput {
            users: users
                .into_iter()
                .map(|user| get_visible_user(user, self.role_namer.clone()))
                .collect(),
        });
    }
}
#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::access_management::role_namer_spy::RoleNamerSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::test_utils::errors_assertion::assert_forbidden_error;
    use crate::users::domain::User;
    use crate::users::interactors::mocks::fake_users_repository::FakeUsersRepository;

    use super::*;

    pub struct CreationResult {
        interactor: ListUsersInteractor,
        repo: Arc<FakeUsersRepository>,
        role_namer: Arc<RoleNamerSpy>,
    }

    const ROLE_NAME: &'static str = "ROLE";

    fn create_interactor() -> CreationResult {
        let repo = Arc::new(FakeUsersRepository::new_with_data(&[
            User {
                id: "1".to_string(),
                name: "user1".to_string(),
                email: "a@email.com".into(),
                password: "password".to_string(),
                role: Box::from(RoleSpy::new_allowed()),
            },
            User {
                id: "2".to_string(),
                name: "user2".to_string(),
                email: "b@email.com".into(),
                password: "password".to_string(),
                role: Box::from(RoleSpy::new_allowed()),
            },
        ]));
        let role_namer = Arc::new(RoleNamerSpy::new_returning(ROLE_NAME.into()));
        let interactor = ListUsersInteractor::new(repo.clone(), role_namer.clone());
        CreationResult {
            interactor,
            repo,
            role_namer,
        }
    }
    #[tokio::test]
    async fn should_refuse_to_list_users_if_user_is_not_allowed() {
        let CreationResult { interactor, .. } = create_interactor();
        let auth = AuthPayloadSpy::new_disallowed("ID".into());

        let result = interactor.execute(&auth).await.unwrap_err();

        assert_forbidden_error(result);
    }
    #[tokio::test]
    async fn should_call_role_namer_for_each_user() {
        let CreationResult {
            interactor,
            repo,
            role_namer,
        } = create_interactor();

        interactor.execute(&allowed_auth()).await.unwrap();

        assert_eq!(
            role_namer.called_with_roles.lock().unwrap().len(),
            repo.get_users().len()
        );
    }
    #[tokio::test]
    async fn should_return_users_with_role_name() {
        let CreationResult { interactor, .. } = create_interactor();
        let result_users = interactor.execute(&allowed_auth()).await.unwrap().users;

        for user in result_users {
            assert_eq!(user.role, ROLE_NAME);
        }
    }

    #[tokio::test]
    async fn should_return_same_user_ids() {
        let CreationResult {
            interactor, repo, ..
        } = create_interactor();

        let users = interactor.execute(&allowed_auth()).await.unwrap().users;

        for user in users {
            assert!(repo.get_by_id(&user.id).await.unwrap().is_some());
        }
    }

    fn allowed_auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ID".into())
    }
}
