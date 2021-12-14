use std::sync::Mutex;

use crate::access_management::{Role, RoleFactory};
use crate::test_utils::access_management::allowed_role::AllowedRole;

pub struct AllowedRoleRoleFactorySpy {
    pub called_with: Mutex<Vec<String>>,
}

impl RoleFactory for AllowedRoleRoleFactorySpy {
    fn is_valid_role_name(&self, _role_name: &str) -> bool {
        true
    }

    fn create_role(&self, role_name: &str) -> Option<Box<dyn Role>> {
        self.called_with.lock().unwrap().push(role_name.to_string());
        Some(Box::new(AllowedRole {}))
    }
}

impl AllowedRoleRoleFactorySpy {
    pub fn new() -> AllowedRoleRoleFactorySpy {
        AllowedRoleRoleFactorySpy {
            called_with: Mutex::new(Vec::new()),
        }
    }
}
