#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene3d = Scene3::new();

    for i in 0..10 {
        for j in 0..10 {
            let cube = cube()
                .scale(5.)
                .translate(vec3(10.1 * i as f64, 10.1 * j as f64, 0.));
            scene3d.add_form(cube);
        }
    }

    let mut plot = scene3d.camera_distance(-100.).to_2d().to_plot();

    println!("Before optimization: {:?}", plot.cost());
    plot = plot.optimize();
    println!("After optimization: {:?}", plot.cost());

    plot.to_svg().save(&svg_filename!());
}
