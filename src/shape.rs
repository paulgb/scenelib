use crate::geom::polygon::Polygon;
use crate::geom::vector::Vector;
use crate::geom::coord::Coord;

use std::f64::consts::PI;

pub fn rect(origin: Coord, size: Vector) -> Polygon {
    Polygon::new(vec![
        origin,
        Coord::new(origin.x, origin.y + size.y),
        origin + size,
        Coord::new(origin.x + size.x, origin.y)
    ])
}

pub fn circle(origin: Coord, radius: f64, divisions: usize) -> Polygon {
    let v = Vector::new(radius, 0.0);
    Polygon::new(
        (0..divisions)
        .map(|i| origin + v.rotate((i as f64 / divisions as f64) * PI * 2.0))
        .collect())
}

pub fn ellipse(origin: Coord, size: Vector, divisions: usize) -> Polygon {
    Polygon::new(
        (0..divisions)
        .map(|i| {
            let radians = (i as f64 / divisions as f64) * PI * 2.0;
            origin + Vector::new(
                size.x * radians.cos(),
                size.y * radians.sin()
            )
        })
        .collect())
}