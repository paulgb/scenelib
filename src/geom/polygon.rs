use crate::geom::types::{Point2f, Vec2f};

use crate::geom::line_segment::LineSegment;
use crate::geom::traits::{Rotate, Translate};
use rstar::{RTreeObject, AABB};

#[derive(Debug)]
pub struct PointLoop(pub Vec<Point2f>);

impl PointLoop {
    pub fn line_segments(&self) -> Vec<LineSegment> {
        let PointLoop(points) = self;
        let mut result = Vec::new();
        if points.len() < 2 {
            return result
        }
        result.push(LineSegment::new(*points.last().unwrap(), *points.first().unwrap()));

        for i in 0..(points.len() - 1) {
            result.push(LineSegment::new(points[i], points[i+1]))
        }

        result
    }
}

// All polygons are closed.
#[derive(Debug)]
pub struct Polygon {
    pub points: PointLoop,
    pub holes: Vec<PointLoop>
}

impl RTreeObject for Polygon
{
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope
    {
        let points: Vec<[f64; 2]> = self.points.0.iter().map(|p| [p.x, p.y]).collect();
        AABB::from_points(&points)
    }
}

impl Polygon {
    pub fn new(points: Vec<Point2f>) -> Polygon {
        Polygon {
            points: PointLoop(points),
            holes: Vec::new()
        }
    }

    pub fn with_holes(points: Vec<Point2f>, holes: Vec<Vec<Point2f>>) -> Polygon {
        Polygon {
            points: PointLoop(points),
            holes: holes.iter().map(|p| PointLoop(p.clone())).collect()
        }
    }

}

impl Rotate for Polygon {
    fn rotate(&self, center: Point2f, radians: f64) -> Polygon {
        Polygon::new(self.points.0.iter().map(|c: &Point2f| c.rotate(center, radians)).collect())
    }
}

impl Translate for Polygon {
    fn translate(&self, dist: Vec2f) -> Polygon {
        Polygon::new(self.points.0.iter().map(|c: &Point2f| c.translate(dist)).collect())
    }
}
