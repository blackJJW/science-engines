use engine_core::prelude::*;

/*
Analytic solution for harmonic oscillator:
- x(t) = cos(wt)        when x(0) = 1, v(0) = 0
- v(t) = -w * sin(wt)   when x(0) = 1, v(0) = 0
*/
fn osc_analytic(w: f64, t: f64) -> (f64, f64) {
    let x = (w * t).cos();      // position
    let v = -w * (w * t).sin(); // velocity
    (x, v)                      // return (x, v)
}

#[test]
fn rk4_oscillator_matches_analytic_reasonably() {
    let w = 2.0; // angular frequency
    let model = HarmonicOscillator { w };
    let integrator = Rk4;

    let t0 = 0.0; // initial time
    let x0 = 1.0; // initial position
    let v0 = 0.0; // initial velocity

    let dt = 0.001; // small step size for accuracy
    let steps = 10_000; // simulate until t = 10.0

    let traj = simulate(&model, &integrator, t0, &[x0, v0], dt, steps);
    
    let (t_end, x_end) = traj.last().unwrap();
    let x_num = x_end[0];
    let v_num = x_end[1];

    let (x_true, v_true) = osc_analytic(w, *t_end);

    let err_x = (x_num - x_true).abs();
    let err_v = (v_num - v_true).abs();

    /*
    Tolerance:
    - With dt = 1e-3 and RK4, errors should be small.
    - If this fails on your machine, loosen slightly (e.g., 1e-3).
    */
    assert!(err_x < 1e-4, "x error too large: {err_x}, x_num={x_num}, x_true={x_true}");
    assert!(err_v < 1e-4, "v error too large: {err_v}, v_num={v_num}, v_true={v_true}");
}