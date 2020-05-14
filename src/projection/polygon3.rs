use crate::geom::types::{Point2f, Point3f};
use crate::geom::polygon::Polygon;
use crate::projection::transform::Transform;

pub struct Polygon3 {
    pub points: Vec<Point3f>
}

impl Polygon3 {
    pub fn scale(&self, scale: f64) -> Polygon3 {
        Polygon3 {
            points: self.points.iter().map(|d: &Point3f| d * scale).collect()
        }
    }

    pub fn apply(&self, transformation: &dyn Transform) -> Polygon3 {
        Polygon3 {
            points: self.points.iter().map(|d: &Point3f| transformation.transform_point(*d)).collect()
        }
    }

    pub fn to_2d(&self) -> Polygon {
        Polygon {
            points: self.points.iter().map(|d| d.xy()).collect()
        }
    }
}

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
}