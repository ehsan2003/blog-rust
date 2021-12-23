use std::sync::Arc;

use chrono::Utc;
use slug::slugify;
use with_deps_proc_macro::WithDeps;

use ApplicationException::*;

use crate::categories::domain::{Category, CategoryId};
use crate::categories::interactors::actions::CREATE_CATEGORY_ACTION;
use crate::categories::interactors::traits::CategoriesRepository;
use crate::errors::{ApplicationException, ApplicationResult};
use crate::utils::{AuthPayload, RandomService};

#[derive(WithDeps)]
pub struct CreateCategoryInteractor {
    repo: Arc<dyn CategoriesRepository>,
    random: Arc<dyn RandomService>,
}

impl CreateCategoryInteractor {
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: CreateCategoryInput,
    ) -> ApplicationResult<CreateCategoryOutput> {
        auth.can_or_fail(CREATE_CATEGORY_ACTION)?;

        let slug = Self::get_slug(&input);

        if self.repo.get_by_slug(&slug).await?.is_some() {
            return Err(DuplicationException {
                value: slug.into(),
                key: "slug".into(),
            });
        }

        self.check_parent_id(&input).await?;

        let category = Category {
            id: CategoryId::new(&self.random.random_id().await?),
            name: input.name,
            description: input.description,
            created_at: Utc::now(),
            slug,
            parent_id: None,
        };
        self.repo.create(&category).await?;
        Ok(Self::create_output(category))
    }

    fn get_slug(input: &CreateCategoryInput) -> String {
        if let Some(s) = &input.slug {
            s.clone()
        } else {
            slugify(input.name.clone())
        }
    }

    async fn check_parent_id(&self, input: &CreateCategoryInput) -> ApplicationResult<()> {
        if let Some(id) = &input.parent_id {
            self.repo.get_by_id_or_fail(&CategoryId::new(&id)).await?;
        }
        Ok(())
    }

    fn create_output(category: Category) -> CreateCategoryOutput {
        CreateCategoryOutput {
            id: category.id.to_string(),
            name: category.name,
            slug: category.slug,
            description: category.description,
            created_at: category.created_at.to_string(),
            parent_id: category.parent_id.map(|id| id.to_string()),
        }
    }
}
#[derive(Debug, Clone)]
pub struct CreateCategoryOutput {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub created_at: String,
    pub parent_id: Option<String>,
}
#[derive(Debug, Clone)]
pub struct CreateCategoryInput {
    pub name: String,
    pub slug: Option<String>,
    pub description: String,
    pub parent_id: Option<String>,
}
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::{DateTime, Duration, Utc};

    use crate::categories::domain::{Category, CategoryId};
    use crate::categories::interactors::actions::CREATE_CATEGORY_ACTION;
    use crate::categories::interactors::test_doubles::fake_categories_repository::FakeCategoriesRepository;
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::crypto::random_service_spy::{RandomServiceSpy, RANDOM_ID};
    use crate::test_utils::errors_assertion::*;

    use super::*;

    make_interactor_setup!(
        CreateCategoryInteractor,
        [
            (
                repo,
                FakeCategoriesRepository::new_with_data(&[existing_category()]),
                FakeCategoriesRepository
            ),
            (random, RandomServiceSpy::new(), RandomServiceSpy)
        ]
    );
    fn valid_input() -> CreateCategoryInput {
        CreateCategoryInput {
            name: "Test Category".to_string(),
            slug: Some("test-category".to_string()),
            description: "Test Category Description".to_string(),
            parent_id: None,
        }
    }

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
    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ID".into())
    }
    #[tokio::test]
    async fn should_throw_error_if_parent_id_does_not_exists() {
        let c = create_interactor();
        let mut input = valid_input();
        input.parent_id = Some("NOT FOUND".into());

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();

        assert_not_found_error(err);
    }
    #[tokio::test]
    async fn should_throw_error_if_slug_is_not_unique() {
        let c = create_interactor();

        let mut input = valid_input();
        input.slug = Some(existing_category().slug);

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();

        assert_duplication_error(err, "slug");
    }
    #[tokio::test]
    async fn should_throw_error_if_user_is_not_allowed() {
        let c = create_interactor();

        let err = c
            .interactor
            .execute(&AuthPayloadSpy::new_disallowed("ID".into()), valid_input())
            .await
            .unwrap_err();

        assert_forbidden_error(err);
    }

    #[tokio::test]
    async fn should_pass_the_appropriate_action_to_auth_payload() {
        let c = create_interactor();
        let auth = AuthPayloadSpy::new_allowed("ID".into());

        c.interactor.execute(&auth, valid_input()).await.unwrap();

        assert_eq!(auth.get_called(), [CREATE_CATEGORY_ACTION]);
    }
    #[tokio::test]

    async fn should_pass_slugifyed_slug_to_repository() {
        let c = create_interactor();

        let mut input = valid_input();
        input.name = "hello world :)".into();
        input.slug = None;

        c.interactor.execute(&auth(), input).await.unwrap();

        assert_eq!(c.repo.get_all().await.unwrap()[1].slug, "hello-world");
    }

    #[tokio::test]
    async fn should_return_the_created_category() {
        let c = create_interactor();

        let input = valid_input();

        let result = c.interactor.execute(&auth(), input.clone()).await.unwrap();

        assert_eq!(result.id, RANDOM_ID);
        assert_eq!(result.name, input.name);
        assert_eq!(result.description, input.description);
        assert!(
            Utc::now() - DateTime::from_str(&result.created_at).unwrap() < Duration::seconds(1)
        );
        assert_eq!(result.slug, input.slug.unwrap());
        assert_eq!(result.parent_id, input.parent_id);
    }
    #[tokio::test]
    async fn should_call_random_id_generator() {
        let c = create_interactor();

        let input = valid_input();

        c.interactor.execute(&auth(), input).await.unwrap();

        c.random.assert_random_id_called();
    }
}
