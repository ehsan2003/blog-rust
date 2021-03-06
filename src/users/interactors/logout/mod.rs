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
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::auth_payload_revoker_spy::AuthRevokerSpy;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;

    use super::*;

    make_interactor_setup!(
        LogoutInteractor,
        [(revoker, AuthRevokerSpy::new(), AuthRevokerSpy)]
    );
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
