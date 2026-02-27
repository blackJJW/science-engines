use crate::ode::model::OdeModel;

/*
Logistic growth:
- dy/dy = r * y * (1 - y/K)
- dim = 1
- x[0] = y 
*/
#[derive(Debug, Clone, Copy)]
pub struct Logistic {
    pub r: f64, // growth rate
    pub k: f64, // carrying capacity
}

impl OdeModel for Logistic {
    fn dim(&self) -> usize {
        1
    }

    fn deriv(&self, _t: f64, x: &[f64], out: &mut [f64]) {
        let y = x[0];
        out[0] = self.r * y * (1.0 - y / self.k);
    }
}