/*
OdeModel: dx/dt = f(t, x)
- dim(): Length of state vector x
- deriv(): Write the dx/dt given (t, x) on a buffer (mimimize allocations)
*/
pub trait OdeModel {
    fn dim(&self) -> usize;
    
    fn deriv(&self, t: f64, x: &[f64], out: &mut [f64]);
}