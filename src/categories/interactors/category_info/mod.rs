use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::categories::domain::{Category, CategoryId};
use crate::categories::interactors::traits::{
    CategoriesRepository, CategoryMeta, CategoryMetaCalculator,
};
use crate::errors::ApplicationException::NotFoundException;
use crate::errors::ApplicationResult;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct CategoryInfoOutput {
    pub direct_posts_count: i32,
    pub children_count: i32,
    pub total_post_count: i32,
    pub parent_id: Option<String>,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub id: String,
}

impl CategoryInfoOutput {
    fn new_from_data(c: Category, m: CategoryMeta) -> Self {
        CategoryInfoOutput {
            id: c.id.to_string(),
            name: c.name.to_string(),
            slug: c.slug.to_string(),
            description: c.description.to_string(),
            parent_id: c.parent_id.map(|s| s.to_string()),

            direct_posts_count: m.direct_posts_count,
            children_count: m.children_count,
            total_post_count: m.total_post_count,
        }
    }
}

#[derive(WithDeps)]
pub struct CategoryInfoInteractor {
    repo: Arc<dyn CategoriesRepository>,
    category_meta_calculator: Arc<dyn CategoryMetaCalculator>,
}

impl CategoryInfoInteractor {
    pub async fn execute(&self, input: CategoryInfoInput) -> ApplicationResult<CategoryInfoOutput> {
        let id: CategoryId = input.id.into();
        let category = self.repo.get_by_id_or_fail(&id).await?;
        let meta = self.get_meta(&id).await?;
        Ok(CategoryInfoOutput::new_from_data(category, meta))
    }

    async fn get_meta(&self, id: &CategoryId) -> ApplicationResult<CategoryMeta> {
        let meta = self.category_meta_calculator.get_meta(&id).await?;
        let meta = match meta {
            None => {
                return Err(NotFoundException("Category meta not found".into()).into());
            }
            Some(s) => s,
        };
        Ok(meta)
    }
}

pub struct CategoryInfoInput {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::categories::domain::{Category, CategoryId};
    use crate::categories::interactors::test_doubles::category_meta_calculator_spy::CategoryMetaCalculatorSpy;
    use crate::categories::interactors::test_doubles::fake_categories_repository::FakeCategoriesRepository;
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::errors_assertion::assert_not_found_error;

    use super::*;

    fn existing_category() -> Category {
        Category {
            id: CategoryId::new("test".into()),
            name: "category".to_string(),
            description: "description of the category".to_string(),
            created_at: Utc::now(),
            slug: "slug".to_string(),
            parent_id: None,
        }
    }

    struct CreationResult {
        interactor: CategoryInfoInteractor,
        repo: std::sync::Arc<FakeCategoriesRepository>,
        category_meta_calculator: std::sync::Arc<CategoryMetaCalculatorSpy>,
    }

    fn create_interactor() -> CreationResult {
        let repo =
            std::sync::Arc::new((FakeCategoriesRepository::new_with_data(&[existing_category()])));
        let category_meta_calculator = std::sync::Arc::new((CategoryMetaCalculatorSpy::default()));
        let interactor =
            CategoryInfoInteractor::new(repo.clone(), category_meta_calculator.clone());
        CreationResult {
            interactor,
            repo,
            category_meta_calculator,
        }
    }

    #[tokio::test]
    async fn should_return_not_found_error_if_category_does_not_exists() {
        let c = create_interactor();

        let input = CategoryInfoInput {
            id: "does not exists".into(),
        };
        let error = c.interactor.execute(input).await.unwrap_err();

        assert_not_found_error(error);
    }

    #[tokio::test]
    async fn should_return_proper_result() {
        let c = create_interactor();

        let result = c.interactor.execute(valid_input()).await.unwrap();

        let returned_meta = c.category_meta_calculator.result.clone().unwrap();
        assert_eq!(
            result,
            CategoryInfoOutput::new_from_data(existing_category(), returned_meta)
        )
    }

    fn valid_input() -> CategoryInfoInput {
        CategoryInfoInput {
            id: existing_category().id.to_string(),
        }
    }

    #[tokio::test]
    async fn should_throw_error_if_meta_is_none() {
        let mut c = create_interactor();
        c.interactor
            .set_category_meta_calculator(Arc::new(CategoryMetaCalculatorSpy::new_returning(None)));

        let err = c.interactor.execute(valid_input()).await.unwrap_err();

        assert_not_found_error(err);
    }
}
