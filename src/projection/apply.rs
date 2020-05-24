use crate::types::vector::Vec3f;
use crate::projection::transform::Transform;
use nalgebra::Translation3;

pub trait Apply {
    fn apply(self, transform: &dyn Transform) -> Self;
}

pub trait ApplyOps {
    fn scale(self, scale: f64) -> Self;
    fn scale3(self, x: f64, y: f64, z: f64) -> Self;
    fn translate(self, x: f64, y: f64, z: f64) -> Self;
}

impl<T> ApplyOps for T
where
    T: Apply,
{
    fn scale(self, scale: f64) -> Self {
        self.apply(&scale)
    }

    fn scale3(self, x: f64, y: f64, z: f64) -> Self {
        self.apply(&Vec3f::new(x, y, z))
    }

    fn translate(self, x: f64, y: f64, z: f64) -> Self {
        self.apply(&Translation3::new(x, y, z))
    }
}
