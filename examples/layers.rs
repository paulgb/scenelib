#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();

    let s1 = square();
    let s2 = square().translate(vec(0.5, 0.5));
    scene.add_poly_with_draw_mode(&s1, pen(1));
    scene.add_poly_with_draw_mode(&s2, pen(2));

    scene.to_svg().save(&svg_filename!());
}
