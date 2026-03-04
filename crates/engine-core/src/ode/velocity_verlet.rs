use super::{integrator::Integrator, model::OdeModel, scratch::Scratch};

/*
Velocity Verlet integrator (sympletic, good long-term energy behavior).
Assumption (for now):
- dim == 2
- state x = [pos, vel]
- model.deriv(t, x, out) returns:
    out[0] = dpos/dt (= vel)
    out[1] = dvel/dt (= acceleration)
Note:
- For best correctness, accelration should depend mainly on position (common in physics).
*/

#[derive(Debug, Clone, Copy, Default)]
pub struct VelocityVerlet;

impl Integrator for VelocityVerlet {
    fn step<M: OdeModel>(
        &self,
        model: &M,              // ODE model
        t: f64,                 // current time
        x: &mut [f64],          // in-place state: [pos, vel]
        dt: f64,                // time step
        scratch: &mut Scratch   // reusable buffers
    ) {
        let n = model.dim();

        assert!(n == 2, "VelocityVerlet currently supports dim == 2 (pos, vel)");
        scratch.ensure_dim(n);

        let pos = x[0];
        let vel = x[1];

        /* compute acceleration a_n from deriv(t, x) */
        model.deriv(t, x, &mut scratch.k1);
        let a_n = scratch.k1[1]; // acceleration at time t

        /* update position: x_{n+1} = x_n + v_n*dt + 0.5*a_n*dt^2 */
        let pos_next = pos + vel * dt + 0.5 * a_n * dt * dt; // next position

        /* compute "half velocity": v_{n+1/2} = v_n + 0.5*a_n*dt */
        let vel_half = vel + 0.5 * a_n * dt;

        scratch.tmp[0] = pos_next;
        scratch.tmp[1] = vel_half;

        /* compute acceleration a_{n+1} using deriv(t+dt, x_next) */
        model.deriv(t + dt, &scratch.tmp, &mut scratch.k2);
        let a_next = scratch.k2[1]; // acceleration at time t + dt

        /* update velocity: v_{n+1} = v_{n+1/2} + 0.5*a_{n+1}*dt */
        let vel_next = vel_half + 0.5 * a_next * dt;

        x[0] = pos_next;
        x[1] = vel_next;
    }
}