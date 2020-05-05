use crate::geom::coord::Coord;
use crate::geom::vector::Vector;
use crate::geom::line_segment::LineSegment;
use crate::geom::traits::{Rotate, Translate};
use rstar::{RTreeObject, AABB};

// All polygons are closed.
pub struct Polygon {
    pub points: Vec<Coord>
}

impl RTreeObject for Polygon
{
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope
    {
        let points: Vec<[f64; 2]> = self.points.iter().map(|p| [p.x, p.y]).collect();
        AABB::from_points(&points)
    }
}

impl Polygon {
    pub fn new(points: Vec<Coord>) -> Polygon {
        Polygon {
            points
        }
    }

    pub fn line_segments(&self) -> Vec<LineSegment> {
        let Polygon {points} = self;
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

impl Rotate<Polygon> for Polygon {
    fn rotate(&self, center: Coord, radians: f64) -> Polygon {
        Polygon::new(self.points.iter().map(|c: &Coord| c.rotate(center, radians)).collect())
    }
}

impl Translate<Polygon> for Polygon {
    fn translate(&self, dist: Vector) -> Polygon {
        Polygon::new(self.points.iter().map(|c: &Coord| c.translate(dist)).collect())
    }
}
