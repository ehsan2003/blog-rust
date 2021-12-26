use std::sync::Mutex;

use crate::categories::domain::CategoryId;
use crate::categories::interactors::traits::{CategoryMeta, CategoryMetaCalculator};
use crate::errors::UnknownResult;

#[derive(Debug)]
pub struct CategoryMetaCalculatorSpy {
    pub result: Option<CategoryMeta>,
    pub calls: Mutex<Vec<CategoryId>>,
}

#[async_trait::async_trait]
impl CategoryMetaCalculator for CategoryMetaCalculatorSpy {
    async fn get_meta(&self, id: &CategoryId) -> UnknownResult<Option<CategoryMeta>> {
        self.calls.lock().unwrap().push(id.clone());
        Ok(self.result.clone())
    }
}

impl CategoryMetaCalculatorSpy {
    pub fn new_returning(result: Option<CategoryMeta>) -> Self {
        Self {
            result,
            calls: Mutex::new(vec![]),
        }
    }
}

impl Default for CategoryMetaCalculatorSpy {
    fn default() -> Self {
        Self {
            result: Some(CategoryMeta::default()),
            calls: Mutex::new(vec![]),
        }
    }
}
