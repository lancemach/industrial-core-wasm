//! Shared, runtime-independent primitive types.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Validity {
    Valid,
    InvalidInput,
    Unsupported,
    CalculationError,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Computation<T> {
    pub value: Option<T>,
    pub validity: Validity,
    pub algorithm_id: String,
    pub algorithm_version: String,
}
