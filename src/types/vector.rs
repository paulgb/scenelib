use wasm_bindgen::prelude::*;

use na::{Vector2, Vector3};
use crate::types::point::Point;


pub trait VectorContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

pub trait VectorActions {}

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

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            inner: self.inner / rhs
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            inner: self.inner * rhs
        }
    }
}


impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            inner: self.inner + rhs.inner
        }
    }
}


pub type Vec3f = Vector3<f64>;

#[wasm_bindgen]
pub fn vec(x: f64, y: f64) -> Vector {
    Vector { inner: [x, y].into() }
}