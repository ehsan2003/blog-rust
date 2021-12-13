use std::sync::Mutex;

use crate::access_management::{Role, RoleFactory};

pub struct UnknownRoleRoleFactorySpy {
    pub called_with: Mutex<Vec<String>>,
}

impl RoleFactory for UnknownRoleRoleFactorySpy {
    fn create_role(&self, role_name: &str) -> Option<Box<dyn Role>> {
        self.called_with.lock().unwrap().push(role_name.to_string());
        None
    }
}

impl UnknownRoleRoleFactorySpy {
    pub fn new() -> UnknownRoleRoleFactorySpy {
        UnknownRoleRoleFactorySpy {
            called_with: Mutex::new(Vec::new()),
        }
    }
}
