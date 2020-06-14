#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();
    let origin = pt(0., 0.);
    let steps = 200;

    let p1 = Polygon::new(
        (0..steps).map(
            |i| {
                let s = steps as f64;
                let i = i as f64;
                origin + (80. + (40. * (PI * 8. * i / s).sin())) * Vector::from_angle((i / s) * PI * 2.)
            }
        ).collect()
    );

    scene.add_poly(&p1);
    scene.add_poly(&circle(100).scale(100.));

    scene.to_svg().save(&svg_filename!());
}
