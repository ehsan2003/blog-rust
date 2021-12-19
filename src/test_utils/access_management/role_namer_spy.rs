use std::sync::Mutex;

use crate::access_management::{Role, RoleNamer};

pub struct RoleNamerSpy {
    pub returning_name: String,
    pub called_with_roles: Mutex<Vec<Box<dyn Role>>>,
}

impl RoleNamer for RoleNamerSpy {
    fn name_role(&self, role: Box<dyn Role>) -> String {
        self.called_with_roles.lock().unwrap().push(role);
        self.returning_name.clone()
    }
}
#[allow(unused)]
impl RoleNamerSpy {
    pub(crate) fn new_returning(name: String) -> RoleNamerSpy {
        Self {
            returning_name: name,
            called_with_roles: Mutex::new(Vec::new()),
        }
    }
}
