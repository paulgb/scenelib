use wasm_bindgen::prelude::*;

use na::{Vector2, Vector3};
use crate::types::point::Point;


pub trait VectorContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

trait VectorActions {}

impl<T> VectorActions for T where T: VectorContainer {}


#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    #[wasm_bindgen(skip)]
    pub inner: Vector2<f64>,
}

impl Vector {
    pub fn slope(&self) -> f64 {
        (self.inner.y) / (self.inner.x)
    }

    pub fn from_angle(radians: f64) -> Vector {
        Vector {
            inner: Vector2::<f64>::new(radians.cos(), radians.sin()),
        }
    }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "vec({}, {})", self.inner.x, self.inner.y)
    }
}

pub type Vec3f = Vector3<f64>;
