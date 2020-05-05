use std::f64::consts::PI;

use scenelib::geom::coord::Coord;
use scenelib::geom::traits::{Rotate, Translate};
use scenelib::geom::vector::Vector;

fn main() {
    let mut scene = scenelib::scene::Scene::new();
    let origin = Coord::new(20., 40.);
    let size = Vector::new(40., 20.);
    let center = origin + size / 2.;

    let s1 = scenelib::shape::square(origin, size);
    scene.add_poly(&scenelib::shape::circle(origin + size * 3., 100., 100));

    for i in 0..40 {        
        let s2 = s1
            .rotate(center + size / 2., i as f64 * (PI / 30.0))
            .translate(size * i as f64 / 10.);
        scene.add_poly(&s2);
    }
    
    scene.to_svg("basic_shapes.svg");
}
