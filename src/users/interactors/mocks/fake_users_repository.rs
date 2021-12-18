use std::sync::Mutex;

use crate::errors::UnknownResult;
use crate::users::domain::User;
use crate::users::interactors::traits::UsersRepository;

pub struct FakeUsersRepository {
    users: Mutex<Vec<User>>,
}

#[async_trait::async_trait]
impl UsersRepository for FakeUsersRepository {
    async fn get_by_id(&self, _id: &str) -> UnknownResult<Option<User>> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.id == _id)
            .map(|user| (*user).clone()))
    }

    async fn get_by_email(&self, _email: &str) -> UnknownResult<Option<User>> {
        Ok(self
            .users
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.email == _email)
            .map(|user| (*user).clone()))
    }

    async fn create(&self, _user: &User) -> UnknownResult<()> {
        self.users.lock().unwrap().push(_user.clone());
        Ok(())
    }

    async fn update(&self, _user: &User) -> UnknownResult<()> {
        let mut users = self.users.lock().unwrap();
        let index = users.iter().position(|user| user.id == _user.id);
        if let Some(index) = index {
            users[index] = _user.clone();
        }
        Ok(())
    }

    async fn delete(&self, _id: &str) -> UnknownResult<()> {
        let mut users = self.users.lock().unwrap();
        let index = users.iter().position(|user| user.id == _id);
        if let Some(index) = index {
            users.remove(index);
        }
        Ok(())
    }

    async fn get_all(&self) -> UnknownResult<Vec<User>> {
        Ok(self.users.lock().unwrap().clone())
    }
}

impl FakeUsersRepository {
    pub fn new_empty() -> Self {
        FakeUsersRepository {
            users: Mutex::new(Vec::new()),
        }
    }
    pub fn new_with_data(users: &[User]) -> Self {
        FakeUsersRepository {
            users: Mutex::new(Vec::from(users)),
        }
    }
    pub fn get_users(&self) -> Vec<User> {
        self.users.lock().unwrap().clone()
    }
}
