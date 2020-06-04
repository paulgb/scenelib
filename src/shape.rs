//! Primitive shape implementations.
//! Shapes are of unit size and are created at the origin;
//! they are represented as `Polygon`s and so can be transformed
//! as needed after creation.

use crate::geom::polygon::Polygon;
use crate::types::{Point, Vector, VectorExtension};

use std::f64::consts::PI;

/// Construct a 2x2 square centered at the origin.
pub fn square() -> Polygon {
    Polygon::new(vec![
        Point::new(-1., -1.),
        Point::new(-1., 1.),
        Point::new(1., 1.),
        Point::new(1., -1.),
    ])
}

/// Construct a unit circle at the origin.
pub fn circle(divisions: usize) -> Polygon {
    Polygon::new(
        (0..divisions)
            .map(|i| Vector::from_angle((i as f64 / divisions as f64) * PI * 2.0).into())
            .collect(),
    )
}
