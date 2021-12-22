use crate::categories::domain::{Category, CategoryId};
use crate::errors::ApplicationException::NotFoundException;
use crate::errors::{ApplicationResult, UnknownResult};

#[async_trait::async_trait]
pub trait CategoriesRepository: Send + Sync {
    async fn get_by_id(&self, id: &CategoryId) -> UnknownResult<Option<Category>>;
    async fn get_all(&self) -> UnknownResult<Vec<Category>>;
    async fn create(&self, category: &Category) -> UnknownResult<Category>;
    async fn update(&self, category: &Category) -> UnknownResult<Category>;
    async fn delete(&self, id: &CategoryId) -> UnknownResult<()>;
    async fn get_by_slug(&self, slug: &str) -> UnknownResult<Option<Category>>;

    async fn get_by_slug_or_fail(&self, slug: &str) -> ApplicationResult<Category> {
        let category = self.get_by_slug(slug).await?;
        category.ok_or_else(|| NotFoundException(format!("Category with slug {} not found", slug)))
    }

    async fn get_by_id_or_fail(&self, id: &CategoryId) -> ApplicationResult<Category> {
        let category = self.get_by_id(id).await?;
        category.ok_or_else(|| {
            NotFoundException(format!("Category with id {} not found", id.to_string()))
        })
    }
}
