use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::categories::interactors::traits::CategoriesRepository;
use crate::categories::interactors::utils::VisibleCategory;
use crate::errors::ApplicationResult;

#[derive(WithDeps)]
pub struct GetAllInteractor {
    repo: Arc<dyn CategoriesRepository>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GetAllOutput {
    pub categories: Vec<VisibleCategory>,
}

impl GetAllInteractor {
    pub async fn execute(&self) -> ApplicationResult<GetAllOutput> {
        let result = self.repo.get_all().await?;
        Ok(GetAllOutput {
            categories: result.into_iter().map(|category| category.into()).collect(),
        })
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
        GetAllInteractor,
        [(
            repo,
            FakeCategoriesRepository::new_with_data(&categories()),
            FakeCategoriesRepository
        )]
    );
    fn categories() -> Vec<Category> {
        vec![
            Category {
                id: CategoryId::new("1"),
                name: "parent".to_string(),
                description: "".to_string(),
                created_at: Utc::now(),
                slug: "slug-parent".to_string(),
                parent_id: None,
            },
            Category {
                id: CategoryId::new("2"),
                name: "child".to_string(),
                description: "".to_string(),
                created_at: Utc::now(),
                slug: "slug-child".to_string(),
                parent_id: Some(CategoryId::new("1")),
            },
            Category {
                id: CategoryId::new("3"),
                name: "child2".to_string(),
                description: "".to_string(),
                created_at: Utc::now(),
                slug: "slug-child2".to_string(),
                parent_id: Some(CategoryId::new("1")),
            },
            Category {
                id: CategoryId::new("4"),
                name: "child of 2".to_string(),
                description: "".to_string(),
                created_at: Utc::now(),
                slug: "slug-child3".to_string(),
                parent_id: Some(CategoryId::new("2")),
            },
        ]
    }

    #[tokio::test]
    async fn should_return_empty_if_no_categories() {
        let mut c = create_interactor();
        c.interactor
            .set_repo(Arc::new(FakeCategoriesRepository::new_empty()));

        let result = c.interactor.execute().await.unwrap();

        assert!(result.categories.is_empty());
    }

    #[tokio::test]
    async fn should_return_the_data() {
        let mut c = create_interactor();
        let result = c.interactor.execute().await.unwrap();

        assert_eq!(result.categories.len(), categories().len());
        assert_eq!(
            result.categories,
            categories()
                .iter()
                .map(|c| (c.clone()).into())
                .collect::<Vec<VisibleCategory>>()
        );
    }
}
