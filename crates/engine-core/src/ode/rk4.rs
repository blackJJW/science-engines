use super::{integrator::Integrator, model::OdeModel, scratch::Scratch};

/*
RK4: (Runge-Kutta 4th order)
- Commonly used as a first integrator because it provies good accuracy and stability.
*/
#[derive(Debug, Clone, Copy, Default)]
pub struct Rk4;

impl Integrator for Rk4 {
    fn step<M: OdeModel>(
        &self,
        model: &M,
        t: f64,
        x: &mut [f64],
        dt: f64,
        scratch: &mut Scratch,
    ) {
        let n = model.dim();
        scratch.ensure_dim(n);

        // k1 = f(t, x)
        model.deriv(t, x, &mut scratch.k1);

        // tmp = x + dt/2 * k1, k2 = f(t + dt/2, tmp)
        for i in 0..n {
            scratch.tmp[i] = x[i] + 0.5 * dt * scratch.k1[i];
        }
        model.deriv(t + 0.5 * dt, &scratch.tmp, &mut scratch.k2);

        // tmp = x + dt/2 * k2, k3 = f(t + dt/2, tmp)
        for i in 0..n {
            scratch.tmp[i] = x[i] + 0.5 * dt * scratch.k2[i];
        }
        model.deriv(t + 0.5 * dt, &scratch.tmp, &mut scratch.k3);

        // tmp = x + dt * k3, k4 = f(t + dt, tmp)
        for i in 0..n {
            scratch.tmp[i] = x[i] + dt * scratch.k3[i];
        }
        model.deriv(t + dt, &scratch.tmp, &mut scratch.k4);

        // x_new = x + dt/6 * (k1 + 2*k2 + 2*k3 + k4)
        for i in 0..n {
            x[i] = x[i] + (dt / 6.0) * (scratch.k1[i] + 2.0 * scratch.k2[i] + 2.0 * scratch.k3[i] + scratch.k4[i]);
        }
    }
}