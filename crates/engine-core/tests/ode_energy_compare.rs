use engine_core::prelude::*;

fn energy(w: f64, x: f64, v: f64) -> f64 {
    0.5 * v * v + 0.5 * (w * w) * x * x
}