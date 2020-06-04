#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    let control_points = 20;
    let resolution = 20;
    let divisions = control_points * resolution;

    let mut noise_maker = NoiseMaker::new(344.);
    noise_maker.x_period(control_points);
    let origin = pt(0., 0.);

    for y in 1..200 {
        let poly = Polygon::new(
            (0..divisions)
                .map(|i| {
                    let r = noise_maker
                        .noise(i as f64 / resolution as f64, y as f64 / resolution as f64);
                    origin
                        + (2. * r + 8.)
                            * Vector::from_angle((i as f64 / divisions as f64) * 2. * PI)
                })
                .collect(),
        );

        let mut p3 = Polygon3::from_poly(&poly);
        p3 = p3.scale(100.).translate(vec3(0., 0., 20. * y as f64));
        scene3d.add_poly(p3);
    }

    let scene = scene3d.camera_distance(-8000.).to_2d();
    scene.to_svg().save(&svg_filename!());
}
