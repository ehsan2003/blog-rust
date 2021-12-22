use std::str::FromStr;

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CategoryId(String);

impl CategoryId {
    pub fn new(id: String) -> Self {
        CategoryId(id)
    }
}

impl ToString for CategoryId {
    fn to_string(&self) -> String {
        self.0.to_string()
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
