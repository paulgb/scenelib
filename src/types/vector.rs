//! Aliases and adds utility methods to `Vector2`.

use crate::types::to_string::ToString;
use na::{Vector2, Vector3};

pub type Vector = Vector2<f64>;

/// Helper methods for 2D vectors.
pub trait VectorExtension {
    /// Compute the slope of this vector.
    fn slope(&self) -> f64;

    /// Construct a unit vector from the given angle in radians.
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

/// Construct a 2D vector.
pub fn vec(x: f64, y: f64) -> Vector {
    Vector::new(x, y)
}
