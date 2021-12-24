use std::sync::Mutex;

use crate::categories::domain::{Category, CategoryId};
use crate::categories::interactors::traits::CategoriesRepository;
use crate::errors::UnknownResult;

pub struct FakeCategoriesRepository {
    pub categories: Mutex<Vec<Category>>,
}

impl FakeCategoriesRepository {
    pub fn new_empty() -> Self {
        Self {
            categories: Mutex::new(Vec::new()),
        }
    }
    pub fn new_with_data(categories: &[Category]) -> Self {
        Self {
            categories: Mutex::new(categories.to_vec()),
        }
    }
}
#[async_trait::async_trait]
impl CategoriesRepository for FakeCategoriesRepository {
    async fn get_by_id(&self, id: &CategoryId) -> UnknownResult<Option<Category>> {
        let categories = self.categories.lock().unwrap();
        let category = categories
            .iter()
            .find(|category| category.id == *id)
            .map(|c| (*c).clone());
        Ok(category)
    }

    async fn get_all(&self) -> UnknownResult<Vec<Category>> {
        let categories = self.categories.lock().unwrap();
        Ok(categories.clone())
    }

    async fn create(&self, category: &Category) -> UnknownResult<Category> {
        let mut categories = self.categories.lock().unwrap();
        categories.push(category.clone());
        Ok(category.clone())
    }

    async fn update(&self, category: &Category) -> UnknownResult<Category> {
        let mut categories = self.categories.lock().unwrap();
        let index = categories.iter().position(|c| c.id == category.id).unwrap();
        categories[index] = category.clone();
        Ok(category.clone())
    }

    async fn get_by_slug(&self, slug: &str) -> UnknownResult<Option<Category>> {
        let categories = self.categories.lock().unwrap();
        let category = categories
            .iter()
            .find(|category| category.slug == slug)
            .map(|c| (*c).clone());
        Ok(category)
    }
}
