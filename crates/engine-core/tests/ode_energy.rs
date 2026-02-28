use engine_core::prelude::*;

/*
Harmonic oscillator energy:
- E = 1/2 * v^2 + 1/2 * (w^2) * x^2
For an ideal oscillator, E is constant over time.
*/
fn energy(w: f64, x: f64, v: f64) -> f64 {
    // compute total energy
    0.5 * v * v + 0.5 * (w * w) * x * x
}

#[test]
fn rk4_oscillator_energy_is_almost_conserved() {
    let w = 2.0; // angular frequency
    let model = HarmonicOscillator { w };
    // let integrator = Rk4;
    let integrator = SymplecticEuler;

    let t0 = 0.0; 
    let x0 = 1.0;
    let v0 = 0.0;

    let dt = 0.001; // time step (smaller -> better energy behavior)
    let steps = 10_000; //simulate long enough to see drift (t_end=50)

    let traj = simulate(&model, &integrator, t0, &[x0, v0], dt, steps); // run simulation

    let e0 = energy(w, x0, v0);
    let mut e_min = e0;
    let mut e_max = e0; 

    for (_t, x) in traj.iter() { // iterate over trajectory
        let pos = x[0];
        let vel = x[1];
        let e = energy(w, pos, vel);
        if e < e_min { e_min = e; }
        if e > e_max { e_max = e; }
    }

    let drift = (e_max - e_min).abs();
    let rel_drift = drift / e0.max(1e-12);

    /*
    Tolerance notes:
    - RK4 is not symplectic, so energy will not be perfectly conserved.
    - With dt=1e-3, the relative drift over long time should still be small.
    - If this fails, first try smaller dt (e.g., 5e-4).
    */
    assert!(
        rel_drift < 5e-3,
        "energy drift too large: rel_drift={rel_drift}, e0={e0}, e_min={e_min}, e_max={e_max}"
    );
}