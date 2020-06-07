#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    let circles = 360;
    let divisions = 120;
    let mut noise_maker = NoiseMaker::new(344.);
    let x_res = 10;
    let y_res = 10;
    noise_maker.x_period(x_res);
    noise_maker.y_period(y_res);
    let origin = pt(0., 0.);

    for y in 0..circles {
        let poly = Polygon::new(
            (0..divisions)
                .map(|i| {
                    let r = noise_maker.noise(
                        (x_res * i) as f64 / divisions as f64,
                        (y_res * y) as f64 / circles as f64,
                    );
                    origin
                        + (5. * r + 8.)
                            * Vector::from_angle((i as f64 / divisions as f64) * 2. * PI)
                })
                .collect(),
        );

        let theta = (y as f64 / circles as f64) * 2. * PI;
        let mut p3 = Polygon3::from_poly(&poly);
        p3 = p3
            .scale(10.)
            .translate(vec3(-200., 0., 0.))
            .rotate_euler(0., theta, 0.);
        scene3d.add_poly_with_pen(
            p3,
            pen(if pseudo_random(y as f64) > 0.4 {
                1
            } else {
                0
            }),
        );
    }

    let scene = scene3d.camera_distance(10000.).to_2d();
    scene.to_svg().axidraw_portrait().save(&svg_filename!());
}
