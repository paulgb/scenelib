use crate::geom::coord::Coord;
use crate::geom::vector::Vector;
use crate::geom::line_segment::LineSegment;
use crate::geom::traits::{Rotate, Translate};

// All polygons are closed.
pub struct Polygon(pub Vec<Coord>);

impl Polygon {
    pub fn line_segments(&self) -> Vec<LineSegment> {
        let Polygon(points) = self;
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
        Polygon(self.0.iter().map(|c: &Coord| c.rotate(center, radians)).collect())
    }
}

impl Translate<Polygon> for Polygon {
    fn translate(&self, dist: Vector) -> Polygon {
        Polygon(self.0.iter().map(|c: &Coord| c.translate(dist)).collect())
    }
}
