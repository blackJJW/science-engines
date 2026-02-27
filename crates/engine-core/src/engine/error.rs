use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    InvalidDt(f64),
    InvalidConfig(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::InvalidDt(dt) => write!(f, "invalid dt: {dt} (must be > 0)"),
            EngineError::InvalidConfig(msg) => write!(f, "invalid config: {msg}"),
        }
    }
}

impl std::error::Error for EngineError {}