/*
Scratch: Temporary buffers reused by the integrator.
- Improves performance and real-time behavior by avoiding new Vec allocations each step.
*/
#[derive(Debug, Clone)]
pub struct Scratch {
    // k1~k4 for RK4, tempral state vector
    pub k1: Vec<f64>,
    pub k2: Vec<f64>,
    pub k3: Vec<f64>,
    pub k4: Vec<f64>,
    pub tmp: Vec<f64>,
}

impl Scratch {
    pub fn new(dim: usize) -> Self {
        Self {
            k1: vec![0.0; dim],
            k2: vec![0.0; dim],
            k3: vec![0.0; dim],
            k4: vec![0.0; dim],
            tmp: vec![0.0; dim],
        }
    }

    // Adjust the size of the buffers if the model's dimesion changes
    pub fn ensure_dim(&mut self, dim: usize) {
        if self.k1.len() != dim {
            self.k1.resize(dim, 0.0);
            self.k2.resize(dim, 0.0);
            self.k3.resize(dim, 0.0);
            self.k4.resize(dim, 0.0);
            self.tmp.resize(dim, 0.0);
        }
    }
}