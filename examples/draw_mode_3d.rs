#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    let cube = cube();

    scene3d.add_form(cube);

    let scene = scene3d.to_2d();
    scene.to_svg().save(&svg_filename!());
}