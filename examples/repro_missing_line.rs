#[macro_use]
extern crate scenelib;
use nalgebra::Perspective3;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    for i in 1..3 {
        for j in 1..3 {
            let mut cube = cube()
                .scale(50.)
                .translate(vec3(i as f64 * 55., j as f64 * 55., 0.));
            scene3d.append(&mut cube.polys);
        }
    }

    let m: Perspective3<f64> = Perspective3::new(1., 3.14 / 8.0, 100.0, 1000.0);

    scene3d = scene3d
        .apply(&isometric_projection())
        .translate(vec3(0., 0., 1000.))
        .apply(&m);

    let scene = scene3d.to_2d_scene();

    scene.to_plot().write_svg(&svg_filename!());
}
