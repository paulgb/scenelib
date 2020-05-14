use nalgebra::{Rotation3, Translation3};
use crate::geom::types::{Point3f, Vec3f};

pub trait Transform {
    fn transform_point(&self, point: Point3f) -> Point3f;
}

impl Transform for Rotation3<f64> {
    fn transform_point(&self, point: Point3f) -> Point3f {
        self * point
    }
}

impl Transform for Translation3<f64> {
    fn transform_point(&self, point: Point3f) -> Point3f {
        self * point
    }
}

impl Transform for f64 {
    fn transform_point(&self, point: Point3f) -> Point3f {
        point * *self
    }
}

impl Transform for Vec3f {
    fn transform_point(&self, point: Point3f) -> Point3f {
        Point3f::new(self.x * point.x, self.y * point.y, self.z * point.z)
    }
}