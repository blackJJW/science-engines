#[derive(Debug, Clone, Default)]
pub struct EngineState {
    /// Simulation time (seconds)
    pub t: f64,
}

#[derive(Debug, Clone)]
pub struct EngineInput {
    pub dt: f64,
}

#[derive(Debug, Clone)]
pub struct EngineOutput {
    pub state: EngineState,
    pub meta: StepMeta,
}

#[derive(Debug, Clone)]
pub struct StepMeta {
    pub step_count: u64,
    pub dt: f64,
}