use std::sync::Arc;

use with_deps_proc_macro::WithDeps;

use crate::categories::interactors::actions::DELETE_RECURSIVE_CATEGORY_ACTION;
use crate::categories::interactors::traits::{CategoriesRepository, CategoryDeletionUtility};
use crate::errors::ApplicationResult;
use crate::utils::AuthPayload;

#[derive(WithDeps)]
pub struct DeleteRecursiveCategoryInteractor {
    repo: Arc<dyn CategoriesRepository>,
    deleter: Arc<dyn CategoryDeletionUtility>,
}

impl DeleteRecursiveCategoryInteractor {
    pub async fn execute(
        &self,
        auth: &(dyn AuthPayload),
        input: DeleteRecursiveInput,
    ) -> ApplicationResult<()> {
        auth.can_or_fail(DELETE_RECURSIVE_CATEGORY_ACTION)?;

        dbg!(auth);

        let id = input.id.into();
        self.repo.get_by_id_or_fail(&id).await?;
        self.deleter.delete_recursive(&id).await?;
        Ok(())
    }
}

pub struct DeleteRecursiveInput {
    pub id: String,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::categories::domain::Category;
    use crate::categories::interactors::actions::DELETE_RECURSIVE_CATEGORY_ACTION;
    use crate::categories::interactors::test_doubles::category_deleter_spy::CategoryDeletionUtilsSpy;
    use crate::categories::interactors::test_doubles::fake_categories_repository::FakeCategoriesRepository;
    use crate::make_interactor_setup;
    use crate::test_utils::access_management::auth_payload_spy::AuthPayloadSpy;
    use crate::test_utils::errors_assertion::{assert_forbidden_error, assert_not_found_error};

    use super::*;

    pub fn existing_category() -> Category {
        Category {
            id: "ID".into(),
            name: "".to_string(),
            description: "".to_string(),
            created_at: Utc::now(),
            slug: "".to_string(),
            parent_id: None,
        }
    }
    make_interactor_setup!(
        DeleteRecursiveCategoryInteractor,
        [
            (
                repo,
                FakeCategoriesRepository::new_with_data(&[existing_category()]),
                FakeCategoriesRepository
            ),
            (
                deleter,
                CategoryDeletionUtilsSpy::new_default(),
                CategoryDeletionUtilsSpy
            )
        ]
    );

    #[tokio::test]
    async fn should_throw_forbidden_error_when_the_user_is_not_allowed() {
        let c = create_interactor();

        let auth = AuthPayloadSpy::new_disallowed("ID".into());
        let err = c
            .interactor
            .execute(&auth, valid_input())
            .await
            .unwrap_err();
        assert_eq!(auth.get_called(), [DELETE_RECURSIVE_CATEGORY_ACTION]);
        assert_forbidden_error(err);
    }

    #[tokio::test]
    async fn should_throw_not_found_error_when_the_category_does_not_exist() {
        let mut c = create_interactor();
        c.interactor.repo = Arc::new(FakeCategoriesRepository::new_with_data(&[]));

        let err = c
            .interactor
            .execute(&auth(), valid_input())
            .await
            .unwrap_err();

        assert_not_found_error(err);
    }

    #[tokio::test]
    async fn should_call_the_deleter_with_the_category_id() {
        let c = create_interactor();

        c.interactor.execute(&auth(), valid_input()).await.unwrap();

        assert_eq!(c.deleter.get_delete_recursive_calls(), ["ID".into()]);
    }

    fn auth() -> AuthPayloadSpy {
        AuthPayloadSpy::new_allowed("ID".into())
    }

    fn valid_input() -> DeleteRecursiveInput {
        DeleteRecursiveInput { id: "ID".into() }
    }
}
