use super::{model::OdeModel, scratch::Scratch};

/*
Integrator: An algorithm that advances (t, x) -> (t+dt, x_new) by a time step dt.
- step(): Updates x in place 
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