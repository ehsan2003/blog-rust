use std::fmt::Debug;

use dyn_clone::DynClone;

pub trait Role: Debug + Send + Sync + DynClone {
    fn can(&self, action: &str) -> bool;
}
dyn_clone::clone_trait_object!(Role);
pub mod variants;
