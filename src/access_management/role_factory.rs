use crate::access_management::Role;

pub trait RoleFactory: Send + Sync {
    fn is_valid_role_name(&self, role_name: &str) -> bool;
    fn create_role(&self, role_name: &str) -> Option<Box<dyn Role>>;
}

pub trait RoleNamer: Send + Sync {
    fn name_role(&self, role: Box<dyn Role>) -> String;
}
