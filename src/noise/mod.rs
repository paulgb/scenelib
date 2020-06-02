use crate::types::{Vector, VectorExtension};
use std::f64::consts::PI;
pub struct NoiseMaker {
    // Periodicity of noise on the x direction.
    x_period: usize,
    // Seed for the x direction.
    x_seed: f64,
    // Seed for the y direction.
    y_seed: f64,
    // Global seed.
    seed: f64,
}

impl NoiseMaker {
    pub fn new(x_period: usize, x_seed: f64, y_seed: f64, seed: f64) -> NoiseMaker {
        NoiseMaker {
            x_period,
            x_seed,
            y_seed,
            seed,
        }
    }

    fn random(&self, x: usize, y: usize) -> f64 {
        let x = x % self.x_period;
        ((x as f64 * self.x_seed + y as f64 * self.y_seed) * 1e10 + self.seed).sin()
    }

    fn random_unit(&self, x: usize, y: usize) -> Vector {
        Vector::from_angle(PI * 2. * self.random(x, y))
    }

    fn smooth_step(v1: f64, v2: f64, w: f64) -> f64 {
        let mut w = w.max(0.).min(1.);
        w = 6. * w.powi(5) - 15. * w.powi(4) + 10. * w.powi(3);
        (1. - w) * v1 + w * v2
    }

    pub fn noise(&self, x: f64, y: f64) -> f64 {
        let xi = x.floor();
        let xf = x - xi;
        let yi = y.floor();
        let yf = y - yi;

        let v00 = Vector::new(xf, yf);
        let v01 = Vector::new(xf, yf - 1.);
        let v10 = Vector::new(xf - 1., yf);
        let v11 = Vector::new(xf - 1., yf - 1.);

        let n00 = v00.dot(&self.random_unit(xi as usize, yi as usize));
        let n01 = v01.dot(&self.random_unit(xi as usize, yi as usize + 1));
        let n10 = v10.dot(&self.random_unit(xi as usize + 1, yi as usize));
        let n11 = v11.dot(&self.random_unit(xi as usize + 1, yi as usize + 1));

        NoiseMaker::smooth_step(
            NoiseMaker::smooth_step(n00, n01, yf),
            NoiseMaker::smooth_step(n10, n11, yf),
            xf,
        )
    }
}
