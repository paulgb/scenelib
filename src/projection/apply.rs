use crate::projection::transform::Transform;
use crate::projection::types3::Vector3;
use nalgebra::{Rotation3, Translation3};

pub trait Apply {
    fn apply(self, transform: &dyn Transform) -> Self;
}

pub trait ApplyOps {
    fn scale(self, scale: f64) -> Self;
    fn scale3(self, v: Vector3) -> Self;
    fn translate(self, v: Vector3) -> Self;
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
