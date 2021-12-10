use crate::utils::RandomService;

pub struct DummyRandomService;

#[async_trait::async_trait]
impl RandomService for DummyRandomService {
    async fn secure_random_password(&self) -> String {
        unimplemented!()
    }

    async fn random_id(&self) -> String {
        unimplemented!()
    }
}

impl DummyRandomService {
    pub fn new() -> Self {
        DummyRandomService {}
    }
}