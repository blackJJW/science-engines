/*
OdeModel: dx/dt = f(t, x)
- dim(): Return the length of the state vector x.
- deriv(): Writes dx/dt for the given (t, x) into an output buffer (to mimimize allocations).
*/
pub trait OdeModel {
    fn dim(&self) -> usize;
    
    fn deriv(&self, t: f64, x: &[f64], out: &mut [f64]);
}