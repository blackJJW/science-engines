pub mod config;
pub mod error;
pub mod types;

use config::EngineConfig;
use error::EngineError;
use types::{EngineInput, EngineOutput, EngineState, StepMeta};

/// Core engine skeleton.
///
/// Desingn goals:
/// - Pure libraty (no I/O)
/// - Deterministic-friendly
/// - Real-time friendly (step with dt, no hidden sleeps)

pub struct Engine {
    config: EngineConfig,
    state: EngineState,
    step_count: u64,
}

impl Engine {
    pub fn new(config: EngineConfig) -> Result<Self, EngineError> {

        config.validate()?;

        Ok(Self {
            state: EngineState::default(),
            config,
            step_count: 0,
        })
    }

    pub fn config(&self) -> &EngineConfig {
        &self.config
    }

    pub fn state(&self) -> &EngineState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut EngineState {
        &mut self.state
    }

    /// Advance the engine by dt seconnds, with given input.
    /// This is intentionally generic
    pub fn step(&mut self, input: &EngineInput) -> Result<EngineOutput, EngineError> {
        if input.dt <= 0.0 {
            return Err(EngineError::InvalidDt(input.dt));
        }

        self.state.t += input.dt;
        self.step_count += 1;

        Ok(EngineOutput {
            state: self.state.clone(),
            meta: StepMeta {
                step_count: self.step_count,
                dt: input.dt,
            },
        })
    }
}