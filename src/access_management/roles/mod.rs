use std::fmt::Debug;

use dyn_clone::DynClone;

pub trait Role: Debug + Send + Sync + DynClone {
    fn can(&self, action: &str) -> bool;
}

pub mod variants;
