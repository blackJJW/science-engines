use engine_core::prelude::*;

/*
Logistic analytic solution:
y(t) = K / (1 + ((K - y0) / y0) * exp(-r * t))
*/
fn logistic_analytic(r: f64, k: f64, y0: f64, t: f64) -> f64 {
    let a = (k - y0) / y0;
    k / (1.0 + a * (-r * t).exp())
}

#[test]
fn rk4_logistic_matches_analytic_reasonably() {
    let model = Logistic { r: 1.2, k: 10.0 };
    let integrator = Rk4;

    let t0 = 0.0;
    let y0 = 0.5;
    let dt = 0.001;
    let steps = 5_000;

    let traj = simulate(&model, &integrator, t0, &[y0], dt, steps);

    let (t_end, x_end) = traj.last().unwrap();
    let y_num = x_end[0];
    let y_true = logistic_analytic(model.r, model.k, y0, *t_end);

    let err = (y_num - y_true).abs();
    assert!(err < 1e-3, "err too large: {err}, y_num={y_num}, y_true={y_true}");
}