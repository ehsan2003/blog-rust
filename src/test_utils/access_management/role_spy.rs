use std::sync::Mutex;

use crate::access_management::Role;

#[derive(Debug)]
pub struct RoleSpy {
    pub can: bool,
    pub calls: Mutex<Vec<String>>,
}

impl Clone for RoleSpy {
    fn clone(&self) -> Self {
        RoleSpy {
            can: self.can,
            calls: Mutex::new(self.calls.lock().unwrap().clone()),
        }
    }
}

impl Role for RoleSpy {
    fn can(&self, _action: &str) -> bool {
        self.can
    }
}
#[allow(unused)]
impl RoleSpy {
    pub fn new_allowed() -> RoleSpy {
        RoleSpy {
            can: true,
            calls: Mutex::new(Vec::new()),
        }
    }
    pub fn new_disallowed() -> RoleSpy {
        RoleSpy {
            can: false,
            calls: Mutex::new(Vec::new()),
        }
    }
}
