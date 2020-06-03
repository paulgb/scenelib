use crate::types::{Vector, VectorExtension};
use std::f64::consts::PI;
pub struct NoiseMaker {
    // Periodicity of noise.
    x_period: Option<usize>,
    y_period: Option<usize>,
    // Seeds.
    x_seed: f64,
    y_seed: f64,
}

fn pseudo_random(seed: f64) -> f64 {
    (seed.sin() * 1e10).fract()
}

impl NoiseMaker {
    pub fn new(seed: f64) -> NoiseMaker {
        let x_seed = pseudo_random(seed) * 1e6;
        let y_seed = pseudo_random(seed + 1.) * 1e6;

        NoiseMaker {
            x_period: None,
            y_period: None,
            x_seed,
            y_seed
        }
    }

    pub fn x_period(&mut self, x_period: usize) -> &mut NoiseMaker {
        self.x_period = Some(x_period);
        self
    }

    pub fn y_period(&mut self, y_period: usize) -> &mut NoiseMaker {
        self.y_period = Some(y_period);
        self
    }

    fn random(&self, mut x: usize, mut y: usize) -> f64 {
        if let Some(xp) = self.x_period {
            x = x % xp;
        }
        if let Some(yp) = self.y_period {
            y = y % yp;
        }
        pseudo_random(x as f64 * self.x_seed + y as f64 * self.y_seed)
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
