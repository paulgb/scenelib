use std::f64::consts::PI;

use scenelib::geom::coord::Coord;
use scenelib::geom::traits::{Rotate, Translate};
use scenelib::geom::vector::Vector;

fn main() {
    let mut scene = scenelib::scene::Scene::new();

    for i in 0..40 {
        let s2 = s1
            .rotate(center + size / 2., i as f64 * (PI / 30.0))
            .translate(size * i as f64 / 10.);
        scene.add_poly(&s2);
    }
    
    scene.to_svg("basic_shapes.svg");
}
