use crate::geom::polygon::Polygon;
use crate::geom::types::{Point2f, Vec2f};
use na::{Rotation2};

use std::f64::consts::PI;

pub fn rect(origin: Point2f, size: Vec2f) -> Polygon {
    Polygon::new(vec![
        origin,
        Point2f::new(origin.x, origin.y + size.y),
        origin + size,
        Point2f::new(origin.x + size.x, origin.y)
    ])
}

pub fn circle(origin: Point2f, radius: f64, divisions: usize) -> Polygon {
    let v = Vec2f::new(radius, 0.0);
    Polygon::new(
        (0..divisions)
        .map(|i| origin + Rotation2::new((i as f64 / divisions as f64) * PI * 2.0) * v)
        .collect())
}

pub fn ellipse(origin: Point2f, size: Vec2f, divisions: usize) -> Polygon {
    Polygon::new(
        (0..divisions)
        .map(|i| {
            let radians = (i as f64 / divisions as f64) * PI * 2.0;
            origin + Vec2f::new(
                size.x * radians.cos(),
                size.y * radians.sin()
            )
        })
        .collect())
}