#[macro_use]
extern crate scenelib;
use nalgebra::Perspective3;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    for i in 0..40 {
        for j in 0..40 {
            let x: f64 = i as f64 * 101.;
            let y: f64 = j as f64 * 101.;
            let height = (i as f64 / 10. * PI).sin() + (j as f64 / 10. * PI).cos() + 2.;

            let mut cube =
                cube()
                    .scale3(vec3(100., 100., 10.))
                    .translate(vec3(x, y, 100. * height));

            scene3d.append(&mut cube.polys);
        }
    }
    let m: Perspective3<f64> = Perspective3::new(1., 3.14 / 8.0, 100.0, 1000.0);
    let scene = scene3d
        .apply(&isometric_projection())
        .scale(1. / 15.)
        .translate(vec3(0., 0., 1000.))
        .apply(&m)
        .to_2d_scene();

    scene.to_plot().write_svg(&svg_filename!());
}
