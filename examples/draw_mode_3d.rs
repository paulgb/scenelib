#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    scene3d.add_form(cube().draw_mode(pen(0)));

    scene3d.add_form(cube().translate(vec3(1.1, 0., 0.)).draw_mode(pen(1)));

    scene3d.add_form(cube().translate(vec3(0., 1.1, 0.)).draw_mode(pen(2)));

    let scene = scene3d.to_2d();
    scene.to_svg().save(&svg_filename!());
}
