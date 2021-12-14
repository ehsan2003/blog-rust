use crate::access_management::Role;

#[derive(Debug, Clone)]
pub struct AllowedRole;

impl Role for AllowedRole {
    fn can(&self, _action: &str) -> bool {
        true
    }
}
