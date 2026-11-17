//! The single runtime-independent API entry point.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EngineInfo {
    pub version: &'static str,
}

#[derive(Default)]
pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self
    }

    pub fn info(&self) -> EngineInfo {
        EngineInfo {
            version: core_version::ENGINE_VERSION,
        }
    }

    pub fn validate_finite(&self, value: f64) -> Result<(), core_errors::CoreError> {
        core_validation::finite(value, "value").map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;

    #[test]
    fn reports_package_version() {
        assert_eq!(Engine::new().info().version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn rejects_non_finite_input() {
        assert!(Engine::new().validate_finite(f64::NAN).is_err());
    }
}
