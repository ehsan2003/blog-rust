use crate::access_management::Role;

pub trait RoleFactory: Send + Sync {
    fn create_role(&self, role_name: &str) -> Option<Box<dyn Role>>;
}

pub trait RoleNamer: Send + Sync {
    fn name_role(&self, role: Box<dyn Role>) -> Option<String>;
}


