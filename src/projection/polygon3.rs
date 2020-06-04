//! Three dimensional polygons.

use crate::geom::polygon::Polygon;
use crate::projection::apply::Apply;
use crate::projection::transform::Transform;
use crate::projection::types3::Point3;
use crate::types::Point;

/// Represents a polygon in terms of 3D points.
/// Generally these points should lie on the same plane, but
/// there is no requirement that they do.
/// 3D polygons do not have holes.
#[derive(Clone)]
pub struct Polygon3 {
    pub points: Vec<Point3>,
}

impl Polygon3 {
    /// Convert this polygon to a 2D polygon by dropping the Z component. Applies the
    /// given perspective.
    pub fn to_2d(&self, perspective: f64) -> Polygon {
        // This drops the z component from each point, but also inverts the y axis because SVG screen
        // coordinates increase going down.
        Polygon::new(
            self.points
                .iter()
                .map(|d| {
                    let denom = (1. - perspective) * d.z + 1.;
                    Point::new(d.x * (1. / denom), -d.y * (1. / denom))
                })
                .collect(),
        )
    }
}

impl Polygon3 {
    /// Construct a new polygon from a list of 3D points.
    pub fn new(points: Vec<Point3>) -> Polygon3 {
        Polygon3 { points }
    }

    /// Construct a new 3D polygon from a 2D polygon.
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
