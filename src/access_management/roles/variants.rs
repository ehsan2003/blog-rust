use super::Role;

#[derive(Debug, Clone)]
pub struct Admin;

impl Role for Admin {
    fn can(&self, _action: &str) -> bool {
        true
    }
}
