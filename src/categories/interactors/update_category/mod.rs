use std::sync::Arc;

use slug::slugify;
use with_deps_proc_macro::WithDeps;

use crate::categories::domain::CategoryId;
use crate::categories::interactors::actions::UPDATE_CATEGORY_ACTION;
use crate::categories::interactors::traits::CategoriesRepository;
use crate::errors::validation::ValidationError;
use crate::errors::ApplicationException::DuplicationException;
use crate::errors::ApplicationResult;
use crate::utils::{AuthPayload, Validatable};

#[derive(WithDeps)]
pub struct UpdateCategoryInteractor {
    repo: Arc<dyn CategoriesRepository>,
}

impl UpdateCategoryInteractor {
    pub(crate) async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: UpdateCategoryInteractorInput,
    ) -> ApplicationResult<()> {
        auth.can_or_fail(UPDATE_CATEGORY_ACTION)?;
        input.validate()?;

        let id: CategoryId = input.id.into();

        if let Some(c) = input.parent_id.clone() {
            self.repo.get_by_id_or_fail(&c.into()).await?;
        }

        let mut category = self.repo.get_by_id_or_fail(&id).await?;

        let slug = input.slug.unwrap_or(slugify(&input.name));

        if let Some(c) = self.repo.get_by_slug(&slug).await? {
            if c.id != id {
                return Err(DuplicationException {
                    value: slug,
                    key: "slug".into(),
                });
            }
        }

        category.slug = slug;
        category.name = input.name;
        category.parent_id = input.parent_id.map(|c| c.into());
        category.description = input.description;
        self.repo.update(&category).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UpdateCategoryInteractorInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub parent_id: Option<String>,
    pub slug: Option<String>,
}

impl Validatable for UpdateCategoryInteractorInput {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(e) = &self.parent_id {
            if e == &self.id {
                return Err(ValidationError {
                    key: "parent_id".into(),
                    value: e.to_string(),
                    message: "circular parent id".into(),
                });
            }
        }
        if self.name.is_empty() {
            return Err(ValidationError {
                key: "name".into(),
                value: self.name.clone(),
                message: "name is empty".into(),
            });
        }
        let cloned = self.slug.clone();
        if let Some(slug) = &cloned {
            if slug.is_empty() {
                return Err(ValidationError {
                    key: "slug".into(),
                    value: slug.clone(),
                    message: "slug is empty".into(),
                });
            }
        }

        if self.id.is_empty() {
            return Err(ValidationError {
                key: "id".into(),
                value: self.id.clone(),
                message: "id is empty".into(),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::categories::domain::{Category, CategoryId};
    use crate::categories::interactors::actions::UPDATE_CATEGORY_ACTION;
    use crate::categories::interactors::test_doubles::fake_categories_repository::FakeCategoriesRepository;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::errors_assertion::{
        assert_duplication_error, assert_forbidden_error, assert_not_found_error,
        assert_validation_error, assert_validation_error_with_key,
    };

    use super::*;

    struct CreationResult {
        interactor: UpdateCategoryInteractor,
        repo: Arc<FakeCategoriesRepository>,
    }

    fn create_interactor() -> CreationResult {
        let arc = Arc::new(FakeCategoriesRepository::new_with_data(&[
            existing_category(),
            another_category(),
        ]));
        let interactor = UpdateCategoryInteractor { repo: arc.clone() };
        CreationResult {
            interactor,
            repo: arc,
        }
    }

    #[tokio::test]
    async fn should_throw_error_if_the_user_does_not_have_the_permission() {
        let c = create_interactor();
        let auth = AuthPayloadSpy::new_disallowed("ID".into());

        let err = c
            .interactor
            .execute(&auth, valid_input())
            .await
            .unwrap_err();

        assert_eq!(auth.get_called(), [UPDATE_CATEGORY_ACTION]);
        assert_forbidden_error(err);
    }

    #[tokio::test]
    async fn should_throw_error_if_the_category_does_not_exists() {
        let c = create_interactor();
        let mut input = valid_input();
        input.id = "not_existing".to_string();

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();

        assert_not_found_error(err);
    }

    #[tokio::test]
    async fn should_throw_not_found_if_the_parent_is_set_and_does_not_exists() {
        let c = create_interactor();
        let mut input = valid_input();
        input.parent_id = Some("not_existing".to_string());

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();

        assert_not_found_error(err);
    }

    #[tokio::test]
    async fn should_throw_if_slug_is_is_duplicate_by_itself() {
        let c = create_interactor();
        let mut input = valid_input();

        input.slug = another_category().slug.into();

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();
        assert_duplication_error(err, "slug");
    }

    #[tokio::test]
    async fn should_throw_if_slug_is_is_duplicate_which_extracted_from_name() {
        let c = create_interactor();
        let mut input = valid_input();

        input.slug = None;
        input.name = "another slug".into();

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();

        assert_duplication_error(err, "slug");
    }

    #[tokio::test]
    async fn should_not_throw_error_when_slug_is_duplicate_but_the_id_is_the_same() {
        let c = create_interactor();
        let mut input = valid_input();

        input.slug = another_category().slug.into();
        input.id = another_category().id.into();

        c.interactor.execute(&auth(), input).await.unwrap();
    }

    #[tokio::test]
    async fn should_store_the_new_properties() {
        let c = create_interactor();
        let input = valid_input();

        c.interactor.execute(&auth(), input.clone()).await.unwrap();

        let category = c.repo.get_by_id(&input.id.into()).await.unwrap().unwrap();
        assert_eq!(category.name, input.name);
        assert_eq!(category.description, input.description);
        assert_eq!(category.parent_id, input.parent_id.map(|id| id.into()));
        assert_eq!(category.slug, input.slug.unwrap());
    }

    #[tokio::test]
    async fn shuld_throw_validation_error_if_the_parent_id_is_the_same_as_id() {
        let c = create_interactor();
        let mut input = valid_input();
        input.parent_id = Some(input.id.clone());

        let err = c.interactor.execute(&auth(), input).await.unwrap_err();
        assert_validation_error_with_key(err, "parent_id");
    }

    #[tokio::test]
    async fn should_throw_validation_error_for_invalid_inputs() {
        let c = create_interactor();
        let inputs = vec![
            UpdateCategoryInteractorInput {
                id: "".to_string(),
                ..valid_input()
            },
            UpdateCategoryInteractorInput {
                name: "".to_string(),
                ..valid_input()
            },
            UpdateCategoryInteractorInput {
                slug: Some("".to_string()),
                ..valid_input()
            },
        ];
        for input in inputs {
            let err = c.interactor.execute(&auth(), input).await.unwrap_err();
            assert_validation_error(err);
        }
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ID".into())
    }

    fn valid_input() -> UpdateCategoryInteractorInput {
        UpdateCategoryInteractorInput {
            id: existing_category().id.to_string(),
            name: "name".to_string(),
            description: "description".to_string(),
            parent_id: None,
            slug: "slug".to_string().into(),
        }
    }

    fn another_category() -> Category {
        Category {
            id: CategoryId::new("another_id"),
            name: "another_name".to_string(),
            description: "another_description".to_string(),
            parent_id: None,
            slug: "another-slug".to_string(),
            created_at: Utc::now(),
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
}
