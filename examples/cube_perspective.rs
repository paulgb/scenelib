#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d = Scene3::new();

    let cube = cube().scale(50.).translate(vec3(10., 0., 0.));
    scene3d.add_form(cube);

    scene3d
        .apply(&isometric_projection())
        .to_2d_scene_with_perspective(0.01)
        .to_plot()
        .write_svg(&svg_filename!());
}
