#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::types::EngineInput;

    #[test]
    fn engine_step_advances_time() {
        let mut eng = Engine::new(EngineConfig::default()).unwrap();
        let out = eng.step(&EngineInput { dt: 0.1 }).unwrap();
        assert!((out.state.t - 0.1).abs() < 1e-12);
        assert_eq!(out.meta.step_count, 1);
    }

    #[test]
    fn engine_rejects_nonpositive_dt() {
        let mut eng = Engine::new(EngienConfig::default()).unwrap();
        let err = eng.step(&EngineInput { dt: 0.0 }).unwrap_err();
        match err {
            EngineError::InvalidDt(_) => {}
            _ => panic!("unexpected error"),
        }
    }
}