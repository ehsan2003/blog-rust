use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub slug: String,
    pub parent_id: Option<CategoryId>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct CategoryId(String);

impl CategoryId {
    pub fn new(id: &str) -> Self {
        CategoryId(id.into())
    }
}

impl ToString for CategoryId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Into<String> for CategoryId {
    fn into(self) -> String {
        self.0
    }
}

impl From<String> for CategoryId {
    fn from(s: String) -> Self {
        CategoryId(s)
    }
}

impl From<&str> for CategoryId {
    fn from(s: &str) -> Self {
        CategoryId(s.to_string())
    }
}
