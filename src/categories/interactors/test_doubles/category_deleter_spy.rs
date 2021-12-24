use std::collections::HashMap;
use std::sync::Mutex;

use crate::categories::domain::{Category, CategoryId};
use crate::categories::interactors::traits::CategoryDeleter;
use crate::errors::UnknownResult;
use crate::utils::DeletionResult;

pub struct CategoryDeleterSpy {
    recursive_deletion_result: DeletionResult,
    replace_deletion_result: DeletionResult,

    delete_recursive_calls: Mutex<Vec<CategoryId>>,
    replace_calls: Mutex<Vec<(CategoryId, CategoryId)>>,
}

#[async_trait::async_trait]
impl CategoryDeleter for CategoryDeleterSpy {
    async fn delete_recursive(&self, id: &CategoryId) -> UnknownResult<DeletionResult> {
        self.delete_recursive_calls.lock().unwrap().push(id.clone());
        Ok(self.recursive_deletion_result)
    }

    async fn replace_with(
        &self,
        id: &CategoryId,
        replacement_id: &CategoryId,
    ) -> UnknownResult<DeletionResult> {
        self.replace_calls
            .lock()
            .unwrap()
            .push((id.clone(), replacement_id.clone()));
        Ok(self.replace_deletion_result)
    }
}

impl CategoryDeleterSpy {
    pub fn new(
        recursive_deletion_result: DeletionResult,
        replace_deletion_result: DeletionResult,
    ) -> Self {
        Self {
            recursive_deletion_result,
            replace_deletion_result,
            delete_recursive_calls: Mutex::new(Vec::new()),
            replace_calls: Mutex::new(Vec::new()),
        }
    }
    pub fn new_default() -> Self {
        Self::new(DeletionResult::Deleted, DeletionResult::Deleted)
    }
}
