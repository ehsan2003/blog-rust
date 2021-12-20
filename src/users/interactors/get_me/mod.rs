use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::access_management::RoleNamer;
use crate::errors::ApplicationResult;
use crate::users::interactors::utils::{get_visible_user, VisibleUser};
use crate::utils::{AuthPayload, AuthPayloadResolver};

#[derive(WithDeps)]
pub struct GetMeInteractor {
    pub auth_resolver: Arc<dyn AuthPayloadResolver>,
    pub role_namer: Arc<dyn RoleNamer>,
}

impl GetMeInteractor {
    pub async fn execute(&self, auth: &(dyn AuthPayload)) -> ApplicationResult<VisibleUser> {
        let user = self.auth_resolver.resolve(auth).await?;
        Ok(get_visible_user(user, self.role_namer.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::access_management::auth_payload_resolver_spy::AuthPayloadResolverSpy;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::access_management::role_namer_spy::RoleNamerSpy;
    use crate::test_utils::access_management::role_spy::RoleSpy;
    use crate::users::domain::User;

    use super::*;

    #[allow(unused)]
    struct CreationResult {
        interactor: GetMeInteractor,
        role_namer: Arc<RoleNamerSpy>,
        auth_resolver: Arc<AuthPayloadResolverSpy>,
    }
    fn create_interactor() -> CreationResult {
        let role_namer = Arc::new(RoleNamerSpy::new_returning("NAME".to_string()));
        let auth_resolver = Arc::new(AuthPayloadResolverSpy::new_returning(user()));
        let interactor = GetMeInteractor::new(auth_resolver.clone(), role_namer.clone());
        CreationResult {
            interactor,
            role_namer,
            auth_resolver,
        }
    }

    fn user() -> User {
        User {
            id: "id".into(),
            email: "a@email.com".to_string(),
            password: "password".to_string(),
            role: Box::from(RoleSpy::new_allowed()),
            name: "name".to_string(),
        }
    }
    const AUTH_ID: &str = "ID";
    #[tokio::test]
    async fn should_pass_payload_to_resolver() {
        let c = create_interactor();
        let interactor = c.interactor;
        let auth_resolver = c.auth_resolver;
        interactor.execute(&auth()).await.unwrap();
        assert_eq!(*auth_resolver.payload_ids.lock().unwrap(), [AUTH_ID]);
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed(AUTH_ID.into())
    }

    #[tokio::test]
    async fn should_return_a_valid_visible_user() {
        let c = create_interactor();

        let visible_user = c.interactor.execute(&auth()).await.unwrap();

        assert_eq!(visible_user.id, user().id);
        assert_eq!(visible_user.name, user().name);
    }
}
