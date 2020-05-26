use crate::types::point::Point;
use crate::types::polygon::Polygon;
use crate::types::vector::Vector;
use wasm_bindgen::prelude::*;

use std::f64::consts::PI;

#[wasm_bindgen]
pub fn rect() -> Polygon {
    Polygon::new(vec![
        Point {
            inner: [-0.5, -0.5].into(),
        },
        Point {
            inner: [-0.5, 0.5].into(),
        },
        Point {
            inner: [0.5, 0.5].into(),
        },
        Point {
            inner: [0.5, -0.5].into(),
        },
    ])
}

#[wasm_bindgen]
pub fn circle(divisions: usize) -> Polygon {
    Polygon::new(
        (0..divisions)
            .map(|i| Vector::from_angle(
                (i as f64 / divisions as f64) * PI * 2.0).into())
            .collect(),
    )
}
