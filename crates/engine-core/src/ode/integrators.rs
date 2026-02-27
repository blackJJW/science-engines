use super::{model::OdeModel, scratch::Scratch};

/*
Integrator: A algorithm for proceeding (t, x) -> (t+dt, x_new) with a dt
- step(): Update x from in-place 
*/
pub trait Integrator {
    fn step<M: OdeModel>(
        &self,
        model: &M,
        t: f64,
        x: &mut [f64],
        dt: f64,
        scratch: &mut Scratch,
    );
}