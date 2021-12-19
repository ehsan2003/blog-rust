use std::fmt::Debug;

use dyn_clone::DynClone;

use crate::errors::ApplicationException::ForBiddenException;
use crate::errors::ApplicationResult;

pub trait Role: Debug + Send + Sync + DynClone {
    fn can(&self, action: &str) -> bool;
    fn can_or_fail(&self, action: &str) -> ApplicationResult<()> {
        if self.can(action) {
            Ok(())
        } else {
            Err(ForBiddenException("access Denied".into()))
        }
    }
}
dyn_clone::clone_trait_object!(Role);
pub mod variants;
