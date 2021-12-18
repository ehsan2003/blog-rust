use std::sync::Arc;

use crate::errors::ApplicationResult;
use crate::utils::{AuthPayload, AuthRevoker};

pub struct LogoutInteractor {
    auth_revoker: Arc<dyn AuthRevoker>,
}

impl LogoutInteractor {
    pub fn new(auth_revoker: Arc<dyn AuthRevoker>) -> Self {
        Self { auth_revoker }
    }

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
        let CreationResult {
            interactor,
            revoker,
        } = create_interactor();

        interactor
            .execute(&AuthPayloadSpy::new_allowed("".into()))
            .await
            .unwrap();

        assert!(revoker.get_payload_ids().len() > 0);
    }
}
