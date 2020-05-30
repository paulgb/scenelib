use crate::types::{Point, PointContainer};

use crate::geom::line_segment::LineSegment;
use rstar::{RTreeObject, AABB};

#[derive(Debug, Clone)]
pub struct PointLoop(pub Vec<Point>);

impl PointLoop {
    pub fn line_segments(&self) -> Vec<LineSegment> {
        self.line_segments_with_pen(0)
    }

    pub fn line_segments_with_pen(&self, pen: usize) -> Vec<LineSegment> {
        let PointLoop(points) = self;
        let mut result = Vec::new();
        if points.len() < 2 {
            return result;
        }
        result.push(LineSegment::new(
            *points.last().unwrap(),
            *points.first().unwrap(),
        ));

        for i in 0..(points.len() - 1) {
            result.push(LineSegment::new_with_pen(points[i], points[i + 1], pen))
        }

        result
    }
}

#[derive(Clone)]
pub struct Polygon {
    pub points: PointLoop,
    pub holes: Vec<PointLoop>,
}

impl PointContainer for PointLoop {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self {
        PointLoop(self.0.into_iter().map(|p| lambda(p)).collect())
    }
}

impl PointContainer for Polygon {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self {
        Polygon {
            points: self.points.apply(lambda),
            holes: self.holes.into_iter().map(|h| h.apply(lambda)).collect(),
        }
    }
}

impl std::fmt::Debug for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut comma = false;
        write!(f, "Polygon::from_coords(vec![")?;

        for point in &self.points.0 {
            if comma {
                write!(f, ",")?;
            }
            write!(f, "({}, {})", point.x, point.y)?;
            comma = true;
        }

        write!(f, "])")
    }
}

impl RTreeObject for Polygon {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let points: Vec<[f64; 2]> = self.points.0.iter().map(|p| [p.x, p.y]).collect();
        AABB::from_points(&points)
    }
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Polygon {
        Polygon {
            points: PointLoop(points),
            holes: Vec::new(),
        }
    }

    pub fn from_coords(coords: Vec<(f64, f64)>) -> Polygon {
        Polygon {
            points: PointLoop(coords.iter().map(|d| Point::new(d.0, d.1)).collect()),
            holes: Vec::new(),
        }
    }

    pub fn with_holes(points: Vec<Point>, holes: Vec<Vec<Point>>) -> Polygon {
        Polygon {
            points: PointLoop(points),
            holes: holes.iter().map(|p| PointLoop(p.clone())).collect(),
        }
    }
}
