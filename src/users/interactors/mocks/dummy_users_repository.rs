use crate::errors::UnknownResult;
use crate::users::domain::User;
use crate::users::interactors::traits::UsersRepository;

pub struct DummyUsersRepository;

#[async_trait::async_trait]
impl UsersRepository for DummyUsersRepository {
    async fn get_by_id(&self, _id: &str) -> UnknownResult<Option<User>> {
        unimplemented!()
    }

    async fn get_by_email(&self, _email: &str) -> UnknownResult<Option<User>> {
        unimplemented!()
    }

    async fn create(&self, _user: &User) -> UnknownResult<()> {
        unimplemented!()
    }

    async fn update(&self, _user: &User) -> UnknownResult<()> {
        unimplemented!()
    }

    async fn delete(&self, _id: &str) -> UnknownResult<()> {
        unimplemented!()
    }
}
