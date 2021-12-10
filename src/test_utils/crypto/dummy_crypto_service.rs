use crate::errors::UnknownResult;
use crate::utils::CryptoService;

struct DummyCryptoService;

#[async_trait::async_trait]
impl CryptoService for DummyCryptoService {
    async fn hash(&self, _data: &str) -> UnknownResult<String> {
        unimplemented!()
    }

    async fn verify(&self, _data: &str, _hash: &str) -> UnknownResult<bool> {
        unimplemented!()
    }
}