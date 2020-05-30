use crate::projection::types3::{Point3, Vector3};
use nalgebra::{Perspective3, Rotation3, Translation3};

pub trait Transform {
    fn transform_point(&self, point: Point3) -> Point3;
}

impl Transform for Rotation3<f64> {
    fn transform_point(&self, point: Point3) -> Point3 {
        self * point
    }
}

impl Transform for Translation3<f64> {
    fn transform_point(&self, point: Point3) -> Point3 {
        self * point
    }
}

impl Transform for Perspective3<f64> {
    fn transform_point(&self, point: Point3) -> Point3 {
        self.project_point(&point)
    }
}

impl Transform for f64 {
    fn transform_point(&self, point: Point3) -> Point3 {
        point * *self
    }
}

impl Transform for Vector3 {
    fn transform_point(&self, point: Point3) -> Point3 {
        Point3::new(self.x * point.x, self.y * point.y, self.z * point.z)
    }
}
