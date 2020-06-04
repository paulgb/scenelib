//! Type aliases and helper constructors for `nalgebra` points
//! and vectors.

pub type Vector3 = na::Vector3<f64>;
pub type Point3 = na::Point3<f64>;

/// Construct a new 3D vector.
pub fn vec3(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3::new(x, y, z)
}

/// Construct a new 3D point.
pub fn pt3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}
