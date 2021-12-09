use std::any::{Any, TypeId};

use crate::access_management::Role;
use crate::access_management::variants;

pub fn name_role(role: Box<dyn Role>) -> Option<String> {
    let mut v = &role as &dyn Any;
    return if v.type_id() == TypeId::of::<Box<variants::Admin>>() {
        Some("admin".to_string())
    } else {
        None
    };
}

pub fn create_role(role_name: &str) -> Option<Box<dyn Role>> {
    match role_name {
        "admin" => Some(Box::from(variants::Admin)),
        _ => None
    }
}

pub trait RoleFactory: Send + Sync {
    fn create_role(&self, role_name: &str) -> Option<Box<dyn Role>>;
}

pub trait RoleNamer: Send + Sync {
    fn name_role(&self, role: Box<dyn Role>) -> Option<String>;
}


