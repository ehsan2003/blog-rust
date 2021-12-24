#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DeletionResult {
    Deleted,
    NotFound,
}
