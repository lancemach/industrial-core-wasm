//! Shared error definitions.

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    #[error("calculation failed: {0}")]
    Calculation(String),
}
