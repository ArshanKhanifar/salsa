use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub value: i32,
}

pub fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 2), 4);
    }
}
