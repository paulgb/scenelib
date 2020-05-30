use crate::geom::polygon::Polygon;
use crate::types::{Point, Vector};
use na::{Rotation2};

use std::f64::consts::PI;

pub fn rect(origin: Point, size: Vector) -> Polygon {
    Polygon::new(vec![
        origin,
        Point::new(origin.x, origin.y + size.y),
        origin + size,
        Point::new(origin.x + size.x, origin.y)
    ])
}

pub fn circle(origin: Point, radius: f64, divisions: usize) -> Polygon {
    let v = Vector::new(radius, 0.0);
    Polygon::new(
        (0..divisions)
        .map(|i| origin + Rotation2::new((i as f64 / divisions as f64) * PI * 2.0) * v)
        .collect())
}

pub fn ellipse(origin: Point, size: Vector, divisions: usize) -> Polygon {
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