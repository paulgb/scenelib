#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();

    let s1 = rect();
    let s2 = rect().translate(vec(0.5, 0.5));
    scene.add_poly_with_pen(&s1, 1);
    scene.add_poly_with_pen(&s2, 2);

    scene.to_plot().write_svg(&svg_filename!());
}
