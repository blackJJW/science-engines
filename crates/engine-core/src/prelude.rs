pub use crate::engine::{
    config::EngineConfig,
    error::EngineError,
    types::{EngineInput, EngineOutput, EngineState, StepMeta},
    Engine,
};

// ODE public API
pub use crate::ode::{
    integrator::Integrator,
    model::OdeModel,
    rk4::Rk4,
    scratch::Scratch,
    simulate::simulate,
};

// Example models
pub use crate::models::{
    logistic::Logistic,
    oscillator::HarmonicOscillator,
};