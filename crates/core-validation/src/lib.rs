//! Common input validation helpers.

use core_errors::CoreError;

pub fn finite(value: f64, name: &str) -> Result<f64, CoreError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(CoreError::InvalidInput(format!("{name} must be finite")))
    }
}

#[cfg(test)]
mod tests {
    use super::finite;

    #[test]
    fn rejects_non_finite_values() {
        assert!(finite(f64::NAN, "sample").is_err());
    }
}
