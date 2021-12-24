use std::sync::Arc;

pub use with_deps_proc_macro::WithDeps;

use crate::categories::interactors::traits::CategoriesRepository;
use crate::categories::interactors::utils::VisibleCategory;
use crate::errors::ApplicationResult;

#[derive(WithDeps)]
pub struct GetBySlugInteractor {
    repo: Arc<dyn CategoriesRepository>,
}

impl GetBySlugInteractor {
    pub async fn execute(&self, slug: &str) -> ApplicationResult<Option<VisibleCategory>> {
        Ok(self
            .repo
            .get_by_slug(slug)
            .await?
            .map(|category| VisibleCategory::from(category)))
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::categories::domain::{Category, CategoryId};
    use crate::categories::interactors::test_doubles::fake_categories_repository::FakeCategoriesRepository;
    use crate::make_interactor_setup;

    use super::*;

    make_interactor_setup!(
        GetBySlugInteractor,
        [(
            repo,
            FakeCategoriesRepository::new_with_data(&[existing_category()]),
            FakeCategoriesRepository
        )]
    );

    fn existing_category() -> Category {
        Category {
            id: CategoryId::new("id"),
            name: "".to_string(),
            description: "".to_string(),
            created_at: Utc::now(),
            slug: "existing-slug".to_string(),
            parent_id: None,
        }
    }

    #[tokio::test]
    async fn should_return_none_if_category_does_not_exists() {
        let c = create_interactor();
        let result = c.interactor.execute("not-existing-slug").await.unwrap();

        assert!(result.is_none());
    }

    #[tokio::test]
    async fn should_return_category_if_exists() {
        let c = create_interactor();
        let result = c
            .interactor
            .execute(&existing_category().slug)
            .await
            .unwrap();

        assert!(result.is_some());
    }
}
