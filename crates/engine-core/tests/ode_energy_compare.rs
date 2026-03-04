use engine_core::prelude::*;

fn energy(w: f64, x: f64, v: f64) -> f64 {
    // /* E = 1/2 v^2 + 1/2 w^2 x^2 */
    0.5 * v * v + 0.5 * (w * w) * x * x
}
/* compute relative energy range: (max(E)-min(E))/E0 */
fn rel_energy_range(traj: &[(f64, Vec<f64>)], w: f64, e0: f64) -> f64 {
    let mut e_min = e0; // /* track min energy */
    let mut e_max = e0; // /* track max energy */

    for (_t, x) in traj.iter() { // /* scan trajectory */
        let e = energy(w, x[0], x[1]); // /* energy at this step */
        if e < e_min { e_min = e; } // /* update min */
        if e > e_max { e_max = e; } // /* update max */
    }

    (e_max - e_min).abs() / e0.max(1e-12) // /* relative range */
}

#[test]
fn verlet_energy_stays_bounded_over_long_run() {
    let w = 2.0; // /* angular frequency */
    let model = HarmonicOscillator { w }; // /* oscillator model */
    let integrator = VelocityVerlet; // /* symplectic integrator */

    let t0 = 0.0; // /* initial time */
    let x0 = 1.0; // /* initial position */
    let v0 = 0.0; // /* initial velocity */

    /* make dt a bit larger to make the test meaningful */
    let dt = 0.01; // /* larger dt => easier to see integrator behavior */
    let steps = 100_000; // /* long run: t_end = 1000 */

    let traj = simulate(&model, &integrator, t0, &[x0, v0], dt, steps); // /* run simulation */

    let e0 = energy(w, x0, v0); // /* initial energy */
    let range = rel_energy_range(&traj, w, e0); // /* bounded oscillation range */

    /*
    Notes:
    - Symplectic methods often keep energy bounded (oscillating around true E).
    - Range depends on dt; if this fails, try smaller dt (e.g., 0.005).
    */
    assert!(
        range < 5e-2, // /* allow some oscillation but not explosion */
        "energy range too large: range={range}, dt={dt}, steps={steps}"
    );
}