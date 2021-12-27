use crate::categories::domain::{Category, CategoryId};
use crate::errors::ApplicationException::NotFoundException;
use crate::errors::{ApplicationResult, UnknownResult};
use crate::utils::DeletionResult;

#[async_trait::async_trait]
pub trait CategoriesRepository: Send + Sync {
    async fn get_by_id(&self, id: &CategoryId) -> UnknownResult<Option<Category>>;
    async fn get_all(&self) -> UnknownResult<Vec<Category>>;
    async fn create(&self, category: &Category) -> UnknownResult<Category>;
    async fn update(&self, category: &Category) -> UnknownResult<Category>;

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

#[async_trait::async_trait]
pub trait CategoryDeletionUtility: Send + Sync {
    async fn delete_recursive(&self, id: &CategoryId) -> UnknownResult<DeletionResult>;
    async fn replace_with(
        &self,
        id: &CategoryId,
        replacement_id: &CategoryId,
    ) -> UnknownResult<DeletionResult>;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CategoryMeta {
    pub direct_posts_count: i32,
    pub children_count: i32,
    pub total_post_count: i32,
}

#[async_trait::async_trait]
pub trait CategoryMetaCalculator: Send + Sync {
    async fn get_meta(&self, id: &CategoryId) -> UnknownResult<Option<CategoryMeta>>;
}
