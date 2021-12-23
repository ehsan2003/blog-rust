use crate::categories::domain::Category;

#[derive(Debug, PartialEq, Clone)]
pub struct VisibleCategory {
    pub id: String,
    pub created_at: String,
    pub name: String,
    pub description: String,
    pub parent_id: Option<String>,
    pub slug: String,
}

impl From<Category> for VisibleCategory {
    fn from(category: Category) -> Self {
        VisibleCategory {
            id: category.id.to_string(),
            created_at: category.created_at.to_rfc2822(),
            name: category.name,
            description: category.description,
            parent_id: category.parent_id.map(|id| id.to_string()),
            slug: category.slug,
        }
    }
}
