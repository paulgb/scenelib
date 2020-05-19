use nalgebra::{Rotation3, Translation3, Perspective3};
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

/*
impl Transform for Matrix4<f64> {
    fn transform_point(&self, point: Point3f) -> Point3f {
        Point3f::from_homogeneous(self * point.to_homogeneous()).unwrap()
    }
}
*/

impl Transform for Perspective3<f64> {
    fn transform_point(&self, point: Point3f) -> Point3f {
        self.project_point(&point)
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