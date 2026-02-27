use super::error::EngineError;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Optional: deterministic seed for any stochastic components
    pub seed: Option<u64>,
    /// Optional: safety cap for dt
    pub max_dt: Option<f64>,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            seed: None,
            max_dt: Some(0.1),
        }
    }
}

impl EngineConfig {
    pub fn validate(&self) -> Result<(), EngineError> {
        if let Some(max_dt) = self.max_dt {
            if max_dt <= 0.0 {
                return Err(EngineError::InvalidConfig(
                    "max_dt must be > 0".to_string(),
                ));
            }
        }
        Ok(())
    }
}