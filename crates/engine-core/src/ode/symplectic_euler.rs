use super::{integrator::Integrator, model::OdeModel, scratch::Scratch};

/*
Symplectic Euler for "position-velocity" style state.
Assumption:
- dim == 2
- x[0] = position, x[1] = velocity
- model.deriv(t, x, out) writes:
  out[0] = dpos/dt (= velocity)
  out[1] = dvel/dt (= acceleration)
*/
#[derive(Debug, Clone, Copy, Default)]
pub struct SymplecticEuler;

impl Integrator for SymplecticEuler {
    fn step<M: OdeModel>(
        &self,
        model: &M,
        t: f64,
        x: &mut [f64],
        dt: f64,
        scratch: &mut Scratch,
    ) {
        let n = model.dim();

        assert!(n == 2, "SymplecticEuler currently supports dim==2 (pos, vel)");
        scratch.ensure_dim(n);

        // compute derivatives at current state: out = f(t, x)
        model.deriv(t, x, &mut scratch.k1); // k1[0]=vel, k1[1]=acc

        let acc = scratch.k1[1]; // acceleration at current position

        /* 1) update velocity first (semi-implicit) */
        x[1] += dt * acc; // v_{n+1} = v_n + dt * a(x_n)

        /* 2) update position using NEW velocity */
        x[0] += dt * x[1]; // x_{n+1} = x_n + dt * v_{n+1}
    }
}