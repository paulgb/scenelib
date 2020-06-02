#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    for i in 0..50 {
        for j in 0..50 {
            let tet = tetrahedron()
                .scale(30.)
                .rotate_euler(i as f64 * PI / 40., j as f64 * PI / 30., 0.)
                .translate(vec3(60. * i as f64, 60. * j as f64, 0.));

            scene3d.add_form(tet);
        }
    }

    let scene = scene3d.to_2d_scene();
    scene.to_plot().write_svg(&svg_filename!());
}
