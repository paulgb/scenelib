#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    for i in 0..40 {
        for j in 0..40 {
            let x: f64 = i as f64 * 101.;
            let y: f64 = j as f64 * 101.;
            let height = (i as f64 / 10. * PI).sin() + (j as f64 / 10. * PI).cos() + 2.;

            let cube = cube()
                .scale3(vec3(100., 100., 10.))
                .translate(vec3(x, y, 100. * height));

            scene3d.add_form(cube);
        }
    }
    let scene = scene3d.scale(1. / 15.).perspective(1.003).to_2d();

    scene.to_svg().save(&svg_filename!());
}
