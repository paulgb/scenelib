use crate::geom::polygon::Polygon;
use crate::geom::types::{Point, Vector};
use na::{Rotation2};
use wasm_bindgen::prelude::*;
use nalgebra::geometry::UnitComplex;
use nalgebra::Vector2;

use std::f64::consts::PI;

#[wasm_bindgen]
pub fn rect(origin: Point, size: Vector) -> Polygon {
    Polygon::new(vec![
        origin,
        Point {inner: [origin.inner.x, origin.inner.y + size.inner.y].into()},
        Point {inner: origin.inner + size.inner},
        Point {inner: [origin.inner.x + size.inner.x, origin.inner.y].into()}
    ])
}

#[wasm_bindgen]
pub fn circle(divisions: usize) -> Polygon {
    Polygon::new(
        (0..divisions)
        .map(|i| Vector::from_angle((i as f64 / divisions as f64) * PI * 2.0).into())
        .collect())
}
