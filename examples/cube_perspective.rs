#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d = Scene3::new();

    for i in 0..10 {
        for j in 0..10 {
            let cube = cube().scale(5.).translate(vec3(10.1 * i as f64, 10.1 * j as f64, 0.));
            scene3d.add_form(cube);        
        }
    }

    scene3d
        .perspective(1.01)
        .to_2d_scene()
        .to_plot()
        .write_svg(&svg_filename!());
}
