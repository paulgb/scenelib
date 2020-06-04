#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();
    let origin = vec(20., 40.);
    let size = vec(40., 20.);

    let s1 = square().scale2(size).translate(origin);
    scene.add_poly(&circle(100).scale(100.).translate(origin + size * 3.));

    for i in 0..40 {
        let s2 = s1
            .clone()
            .rotate(i as f64 * (PI / 50.0))
            .translate(size * i as f64 / 4.);
        scene.add_poly(&s2);
    }

    scene.to_svg().save(&svg_filename!());
}
