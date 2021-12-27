use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::categories::interactors::actions::REPLACE_CATEGORY_ACTION;
use crate::categories::interactors::traits::{CategoriesRepository, CategoryDeletionUtility};
use crate::errors::ApplicationResult;
use crate::utils::AuthPayload;

pub struct ReplaceCategoryInput {
    pub id: String,
    pub replacement_id: String,
}

#[derive(WithDeps)]
struct ReplaceCategoryInteractor {
    repo: Arc<dyn CategoriesRepository>,
    deleter: Arc<dyn CategoryDeletionUtility>,
}

impl ReplaceCategoryInteractor {
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: ReplaceCategoryInput,
    ) -> ApplicationResult<()> {
        auth.can_or_fail(REPLACE_CATEGORY_ACTION)?;
        let source = self.repo.get_by_id_or_fail(&input.id.into()).await?;
        let replacement = self
            .repo
            .get_by_id_or_fail(&input.replacement_id.into())
            .await?;
        self.deleter
            .replace_with(&source.id, &replacement.id)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;

    use crate::categories::domain::Category;
    use crate::categories::interactors::actions::REPLACE_CATEGORY_ACTION;
    use crate::categories::interactors::test_doubles::category_deleter_spy::CategoryDeletionUtilsSpy;
    use crate::categories::interactors::test_doubles::fake_categories_repository::FakeCategoriesRepository;
    use crate::categories::interactors::traits::{CategoriesRepository, CategoryDeletionUtility};
    use crate::errors::ApplicationException::NotFoundException;
    use crate::errors::ApplicationResult;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::errors_assertion::{assert_forbidden_error, assert_not_found_error};
    use crate::utils::AuthPayload;

    use super::*;

    fn replacement_category() -> Category {
        Category {
            id: "replacement_id".to_string().into(),
            name: "".to_string(),
            description: "".to_string(),
            created_at: Utc::now(),
            slug: "".to_string(),
            parent_id: None,
        }
    }

    fn source_category() -> Category {
        Category {
            id: "source".into(),
            name: "".to_string(),
            description: "".to_string(),
            created_at: Utc::now(),
            slug: "".to_string(),
            parent_id: None,
        }
    }

    struct CreationResult {
        interactor: ReplaceCategoryInteractor,
        repo: Arc<FakeCategoriesRepository>,
        replacer: Arc<CategoryDeletionUtilsSpy>,
    }

    fn create_interactor() -> CreationResult {
        let repo = Arc::new(FakeCategoriesRepository::new_with_data(&[
            source_category(),
            replacement_category(),
        ]));
        let replacer = Arc::new(CategoryDeletionUtilsSpy::new_default());
        let interactor = ReplaceCategoryInteractor::new(repo.clone(), replacer.clone());
        CreationResult {
            interactor,
            repo,
            replacer,
        }
    }

    #[tokio::test]
    async fn should_throw_not_found_when_the_source_does_not_exists() {
        let mut c = create_interactor();
        let repo = Arc::new(FakeCategoriesRepository::new_with_data(&[
            replacement_category(),
        ]));
        c.interactor.set_repo(repo.clone());

        let err = c
            .interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();

        assert_not_found_error(err);
    }

    fn valid_input() -> ReplaceCategoryInput {
        ReplaceCategoryInput {
            id: source_category().id.to_string(),
            replacement_id: replacement_category().id.into(),
        }
    }

    #[tokio::test]
    async fn should_throw_not_found_when_replacement_does_not_exists() {
        let mut c = create_interactor();
        c.interactor
            .set_repo(Arc::new(FakeCategoriesRepository::new_with_data(&[
                source_category(),
            ])));

        let err = c
            .interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();

        assert_not_found_error(err);
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ID".into())
    }

    #[tokio::test]
    async fn should_throw_if_user_has_not_the_permission_for_replacing_a_category() {
        let c = create_interactor();

        let auth = AuthPayloadSpy::new_disallowed("ID".into());
        let err = c
            .interactor
            .execute(&auth, valid_input())
            .await
            .unwrap_err();

        assert_eq!(auth.get_called(), &[REPLACE_CATEGORY_ACTION]);
        assert_forbidden_error(err);
    }

    #[tokio::test]
    async fn should_pass_proper_ids_to_replacer() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(
            c.replacer.get_replace_calls(),
            &[(source_category().id, replacement_category().id)]
        );
    }
}
