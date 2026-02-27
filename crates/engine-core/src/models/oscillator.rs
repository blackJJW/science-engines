use crate::ode::model::OdeModel;

/*
Harmonic oscillator (spring):
x'' + w^2 * x = 0

State:
x[0] = position
x[1] = velocity

dx/dt = v
dv/dt = -w^2 * x
*/
#[derive(Debug, Clone, Copy)]
pub struct HarmonicOscillator {
    pub w: f64, // angular frequency
}

impl OdeModel for HarmonicOscillator {
    fn dim(&self) -> usize {
        2
    }

    fn deriv(&self, _t: f64, x: &[f64], out: &mut [f64]) {
        let pos = x[0];
        let vel = x[1];

        out[0] = vel;
        out[1] = -(self.w * self.w) * pos;
    }
}