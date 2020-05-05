use std::f64::consts::{PI, E};

use scenelib::geom::coord::Coord;
use scenelib::geom::polygon::Polygon;
use scenelib::geom::line_segment::LineSegment;
use scenelib::geom::vector::Vector;

const PI2: f64 = PI * 2.;

fn main() {
    let mut scene = scenelib::scene::Scene::new();
    let origin = Coord::new(0., 0.);


    let mut pos = origin;

    for i in 0..100 {
        let y = i as f64 * 3. - 200.;
        scene.add_segment(LineSegment::new(
            Coord::new(-300., y),
            Coord::new(300., y + 20.)
        ))
    }

    let mut sp1: Vec<Coord> = Vec::new();
    let mut sp2: Vec<Coord> = Vec::new();

    for i in 10..400 {
        let angle = (i as f64 * (PI2 / 5.)).sqrt();
        pos = pos + Vector::from_angle(angle) * 10.;
        let p1 = pos + Vector::from_angle(angle + PI / 4.) * 20.;
        let p2 = pos + Vector::from_angle(angle - PI / 4.) * 20.;

        sp1.push(p1);
        sp2.push(p2);
    }

    sp2.reverse();
    sp1.append(&mut sp2);
    scene.add_poly(&Polygon::new(sp1));
    
    scene.to_svg("spiral.svg");
}
