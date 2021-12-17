use std::sync::Arc;

use crate::access_management::RoleNamer;
use crate::users::domain::User;

#[derive(Debug, Clone)]
pub struct VisibleUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

pub fn get_visible_user(user: User, role_namer: Arc<dyn RoleNamer>) -> VisibleUser {
    VisibleUser {
        id: user.id,
        name: user.name,
        email: user.email,
        role: role_namer.name_role(user.role).unwrap().into(),
    }
}
