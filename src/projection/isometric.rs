//! Implements isometric projection as a rotation.

use nalgebra::{Rotation3, Vector3};
use std::f64::consts::PI;

/// Returns a 3D rotation that achieves an isometric projection.
pub fn isometric_projection() -> Rotation3<f64> {
    // Rotate around vertical axis, i.e. face edge of cube.
    let r1 = Rotation3::from_scaled_axis(Vector3::z() * PI / 4.);

    // Rotate around x axis, i.e. face top corner of cube.
    // This is a rotation by the Magic Angle.
    // https://en.wikipedia.org/wiki/Magic_angle
    let angle = (1. as f64 / (3. as f64).sqrt()).acos();
    let r2 = Rotation3::from_scaled_axis(Vector3::x() * -angle);

    r2 * r1
}
