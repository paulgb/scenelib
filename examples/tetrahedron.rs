#[macro_use]
extern crate scenelib;
use nalgebra::{Rotation3, Translation3};
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    for i in 0..50 {
        for j in 0..50 {
            let tet = tetrahedron()
                .apply(&30.)
                .apply(&Rotation3::from_euler_angles(
                    i as f64 * PI / 40.,
                    j as f64 * PI / 30.,
                    0.,
                ))
                .apply(&Translation3::from(vec3(
                    60. * i as f64,
                    60. * j as f64,
                    0.,
                )));

            scene3d.add_form(tet);
        }
    }

    let scene = scene3d.apply(&isometric_projection()).to_2d_scene();
    scene.to_plot().write_svg(&svg_filename!());
}
