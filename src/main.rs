mod geom;
mod scene;
mod shape;

use std::f64::consts::PI;

use crate::geom::coord::Coord;
use crate::geom::traits::{Rotate, Translate};
use crate::geom::vector::Vector;

fn main() {
    let mut scene = crate::scene::Scene::new();
    let origin = Coord::new(0., 0.);
    let size = Vector::new(40., 40.);
    let center = origin + size / 2.;

    let s1 = crate::shape::square(origin, size);
    
    scene.add_poly(&crate::shape::circle(origin + size * 3., 100., 100));

    for i in 0..40 {        
        let s2 = s1
            .rotate(center + size / 2.01, i as f64 * (PI / 30.0))
            .translate(size * i as f64 / 10.);
        scene.add_poly(&s2);
    }
    
    scene.to_svg("result.svg");
}
