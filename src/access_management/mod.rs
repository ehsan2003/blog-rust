pub use roles::Role;
pub use roles::variants;

mod roles {
    use std::fmt::Debug;
    use dyn_clone::DynClone;

    pub trait Role: Debug + Send + Sync + DynClone {
        fn can(&self, action: &str) -> bool;
    }

    pub mod variants {
        use super::Role;

        #[derive(Debug, Clone)]
        pub struct Admin;

        impl Role for Admin {
            fn can(&self, _action: &str) -> bool {
                true
            }
        }
    }
}