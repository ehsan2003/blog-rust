use std::sync::Mutex;

use crate::access_management::{Role, RoleFactory};

pub struct RoleFactorySpy {
    returning_role: Option<Box<dyn Role>>,
    is_valid_calls: Mutex<Vec<String>>,
    create_role: Mutex<Vec<String>>,
}

impl RoleFactory for RoleFactorySpy {
    fn is_valid_role_name(&self, role_name: &str) -> bool {
        self.is_valid_calls.lock().unwrap().push(role_name.into());
        self.returning_role.is_some()
    }

    fn create_role(&self, role_name: &str) -> Option<Box<dyn Role>> {
        self.create_role.lock().unwrap().push(role_name.into());
        self.returning_role.clone()
    }
}
#[allow(unused)]
impl RoleFactorySpy {
    pub fn new(returning_role: Option<Box<dyn Role>>) -> Self {
        RoleFactorySpy {
            returning_role,
            is_valid_calls: Mutex::new(vec![]),
            create_role: Mutex::new(vec![]),
        }
    }

    pub fn get_is_valid_calls(&self) -> Vec<String> {
        self.is_valid_calls.lock().unwrap().clone()
    }

    pub fn get_create_role_calls(&self) -> Vec<String> {
        self.create_role.lock().unwrap().clone()
    }
}
