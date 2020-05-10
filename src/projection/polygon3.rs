use crate::geom::types::{Point2f, Point3f};
use crate::geom::polygon::Polygon;
use na::{Orthographic3, Perspective3};

pub struct Polygon3 {
    pub points: Vec<Point3f>
}

pub trait Projection {
    fn project(&self, point: &Point3f) -> Point2f;

    fn distance(&self, point: &Point3f) -> f64;
}

/*
impl Projection for Orthographic3<f64> {
    fn project(&self, point: &Point3f) -> Point2f {
        self.project_point(point).xy()
    }
}

impl Projection for Perspective3<f64> {
    fn project(&self, point: &Point3f) -> Point2f {
        self.project_point(point).xy()
    }
}
*/

impl Polygon3 {
    pub fn new(points: Vec<Point3f>) -> Polygon3 {
        Polygon3 {points}
    }

    pub fn from_poly(poly: &Polygon) -> Polygon3 {
        let points: Vec<Point3f> = poly.points.iter().map(
            |p: &Point2f| Point3f::new(p.x, p.y, 0.0)).collect();
        Polygon3 {
            points
        }
    }

    pub fn project_to_poly(&self, proj: &dyn Projection) -> Polygon {
        Polygon {
            points: self.points.iter().map(|p| proj.project(p)).collect()
        }
    }
}