#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

struct NoiseMaker {
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

        let v00 = vec(xf, yf);
        let v01 = vec(xf, yf - 1.);
        let v10 = vec(xf - 1., yf);
        let v11 = vec(xf - 1., yf - 1.);

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

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    let control_points = 20;
    let resolution = 20;
    let divisions = control_points * resolution;

    let noise_maker = NoiseMaker::new(control_points, 36., 74., 0.85);
    let origin = pt(0., 0.);

    for y in 1..200 {
        let poly = Polygon::new(
            (0..divisions)
                .map(|i| {
                    let r = noise_maker.noise(i as f64 / resolution as f64, y as f64);
                    origin + (2. * r + 8.) * Vector::from_angle((i as f64 / divisions as f64) * 2. * PI)
                })
                .collect(),
        );

        let mut p3 = Polygon3::from_poly(&poly);
        p3 = p3.scale(100.).translate(vec3(0., 0., 20. * y as f64));
        scene3d.add_poly(p3);
    }

    let scene = scene3d.perspective(1.00013).to_2d();
    scene.to_svg().save(&svg_filename!());
}
