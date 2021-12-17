use crate::access_management::Role;

pub trait AuthPayload: Role {
    fn get_user_id(&self) -> String;
}
