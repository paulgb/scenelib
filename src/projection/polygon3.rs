use crate::geom::polygon::Polygon;
use crate::projection::types3::Point3;
use crate::projection::apply::Apply;
use crate::projection::transform::Transform;
use crate::types::Point;

#[derive(Clone)]
pub struct Polygon3 {
    pub points: Vec<Point3>,
}

impl Polygon3 {
    pub fn scale(mut self, scale: f64) -> Polygon3 {
        for d in self.points.iter_mut() {
            *d = *d * scale
        }
        self
    }

    pub fn to_2d(&self) -> Polygon {
        // This drops the z component from each point, but also inverts the y axis because SVG screen
        // coordinates increase going down.
        Polygon::new(self.points.iter().map(|d| Point::new(d.x, -d.y)).collect())
    }
}

impl Polygon3 {
    pub fn new(points: Vec<Point3>) -> Polygon3 {
        Polygon3 { points }
    }

    pub fn from_poly(poly: &Polygon) -> Polygon3 {
        let points: Vec<Point3> = poly
            .points
            .0
            .iter()
            .map(|p: &Point| Point3::new(p.x, p.y, 0.0))
            .collect();
        Polygon3 { points }
    }
}

impl Apply for Polygon3 {
    fn apply(mut self, transformation: &dyn Transform) -> Polygon3 {
        for d in self.points.iter_mut() {
            *d = transformation.transform_point(*d)
        }
        self
    }
}
