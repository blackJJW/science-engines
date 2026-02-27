use super::{integrator::Integrator, model::OdeModel, scratch::Scratch};

/*
simulate():
- Integrates for 'steps' steps starting from (t0, x0).
- Returns the trajectory as a Vec of results.
*/
pub fn simulate<M: OdeModel, I: Integrator>(
    model: &M,
    integrator: &I,
    t0: f64,
    x0: &[f64],
    dt: f64,
    steps: usize,
) -> Vec<(f64, Vec<f64>)> {
    let n = model.dim();
    assert_eq!(x0.len(), n, "x0 length must match model.dim()");
    assert!(dt > 0.0, "dt must be > 0");

    let mut scratch = Scratch::new(n);

    let mut t = t0;
    let mut x = x0.to_vec();

    let mut out = Vec::with_capacity(steps + 1);

    // store initial state
    out.push((t, x.clone()));

    for _ in 0..steps {
        integrator.step(model, t, &mut x, dt, &mut scratch);
        t += dt;
        out.push((t, x.clone()));
    }

    out
}