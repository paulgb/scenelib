//! Generic implementation of common point/shape transformations.

use crate::projection::transform::Transform;
use crate::projection::types3::Vector3;
use nalgebra::{Rotation3, Translation3};

/// Trait that indicates that a `Transform` can be applied to
/// a struct. This generally means that the struct is a data
/// structure containing one or more 3D points, and that the
/// transformation will be applied to all of them.
pub trait Apply {
    /// Apply the given transformation to this struct.
    fn apply(self, transform: &dyn Transform) -> Self;
}

/// Provides a number of 3D transformations.
/// Scale and rotation commands operate relative to the origin, so if they
/// are applied *after* a translation, the translation itself will appear
/// to be scaled or rotated. Unless this is desired, shapes should be
/// constructed around the origin and scaled or rotated there *before* being
/// translated into their final position.
pub trait ApplyOps {
    /// Scale every point relative to the origin by a uniform amount.
    fn scale(self, scale: f64) -> Self;
    /// Scale every point relative to the origin by a different amount in each direction.
    fn scale3(self, v: Vector3) -> Self;
    /// Move the points by the given vector.
    fn translate(self, v: Vector3) -> Self;
    /// Rotate all of the points around the origin.
    fn rotate_euler(self, roll: f64, pitch: f64, yaw: f64) -> Self;
}

impl<T> ApplyOps for T
where
    T: Apply,
{
    fn scale(self, scale: f64) -> Self {
        self.apply(&scale)
    }

    fn scale3(self, v: Vector3) -> Self {
        self.apply(&v)
    }

    fn translate(self, v: Vector3) -> Self {
        self.apply(&Translation3::new(v.x, v.y, v.z))
    }

    fn rotate_euler(self, roll: f64, pitch: f64, yaw: f64) -> Self {
        let transform = Rotation3::from_euler_angles(roll, pitch, yaw);
        self.apply(&transform)
    }
}
