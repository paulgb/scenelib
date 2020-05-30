use crate::geom::polygon::Polygon;
use crate::types::{Point, Vector, VectorExtension};

use std::f64::consts::PI;

pub fn rect() -> Polygon {
    Polygon::new(vec![
        Point::new(-1., -1.),
        Point::new(-1., 1.),
        Point::new(1., 1.),
        Point::new(1., -1.),
    ])
}

pub fn circle(divisions: usize) -> Polygon {
    Polygon::new(
        (0..divisions)
            .map(|i| Vector::from_angle((i as f64 / divisions as f64) * PI * 2.0).into())
            .collect(),
    )
}
