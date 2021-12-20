use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::errors::ApplicationResult;
use crate::utils::{AuthPayload, AuthRevoker};

#[derive(WithDeps)]
pub struct LogoutInteractor {
    auth_revoker: Arc<dyn AuthRevoker>,
}

impl LogoutInteractor {
    pub async fn execute(&self, auth: &(dyn AuthPayload)) -> ApplicationResult<()> {
        self.auth_revoker.revoke_auth_payload(auth).await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::test_utils::access_management::auth_payload_revoker_spy::AuthRevokerSpy;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;

    use super::*;

    struct CreationResult {
        interactor: LogoutInteractor,
        revoker: Arc<AuthRevokerSpy>,
    }
    fn create_interactor() -> CreationResult {
        let revoker = Arc::new(AuthRevokerSpy::new());
        let interactor = LogoutInteractor::new(revoker.clone());
        CreationResult {
            interactor,
            revoker,
        }
    }

    #[tokio::test]
    async fn should_pass_payload_to_revoker() {
        let c = create_interactor();

        c.interactor
            .execute(&AuthPayloadSpy::new_allowed("".into()))
            .await
            .unwrap();

        assert!(c.revoker.get_payload_ids().len() > 0);
    }
}
