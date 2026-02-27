use engine_core::prelude::*;

// Basic smoke test: Engine can be created and stepped 
#[test]
fn engie_step_advances_time() {
    let mut eng = Engine::new(EngineConfig::default()).expect("engine init");
    let out = eng.step(&EngineInput { dt: 0.1 }).expect("engine step");

    assert!((out.state.t - 0.1).abs() < 1e-12);
    assert_eq!(out.meta.step_count, 1);
}

// dt validation test
#[test]
fn engine_rejects_nonpositive_dt() {
    let mut eng = Engine::new(EngineConfig::default()).expect("engine init");
    let err = eng.step(&EngineInput { dt: 0.0 }).unwrap_err();

    match err {
        EngineError::InvalidDt(dt) => { /* expected*/ }
        _ => panic!("unexpected error: {err:?}"),
    }
}