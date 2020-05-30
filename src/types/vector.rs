use crate::types::point::Point;
use na::{Vector2, Vector3};
use crate::types::to_string::ToString;

pub trait VectorContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

pub trait VectorActions {}

impl<T> VectorActions for T where T: VectorContainer {}

pub type Vector = Vector2<f64>;

pub trait VectorExtension {
    fn slope(&self) -> f64;
    fn from_angle(radians: f64) -> Vector;
}

impl VectorExtension for Vector {
    fn slope(&self) -> f64 {
        (self.y) / (self.x)
    }

    fn from_angle(radians: f64) -> Vector {
        Vector::new(radians.cos(), radians.sin())
    }
}

impl ToString for Vector {
    fn to_string(&self) -> String {
        format!("vec({}, {})", self.x, self.y)
    }
}

pub type Vec3f = Vector3<f64>;

pub fn vec(x: f64, y: f64) -> Vector {
    Vector::new(x, y)
} 