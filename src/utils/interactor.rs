use crate::errors::{ApplicationException, ApplicationResult};

#[async_trait::async_trait]
pub trait Interactor<Input, Output = ()> {
    async fn execute(&self, input: Input) -> ApplicationResult<Output>;
}