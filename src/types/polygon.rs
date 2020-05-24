use wasm_bindgen::prelude::*;

use crate::types::point::Point;
use crate::types::line_segment::LineSegment;
use rstar::{RTreeObject, AABB};

#[derive(Debug)]
pub struct PointLoop(pub Vec<Point>);

impl PointLoop {
    pub fn line_segments(&self) -> Vec<LineSegment> {
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
            result.push(LineSegment::new(points[i], points[i + 1]))
        }

        result
    }
}
#[wasm_bindgen]
pub struct Polygon {
    #[wasm_bindgen(skip)]
    pub points: PointLoop,
    #[wasm_bindgen(skip)]
    pub holes: Vec<PointLoop>,
}

impl std::fmt::Debug for Polygon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut comma = false;
        write!(f, "Polygon::from_coords(vec![")?;

        for point in &self.points.0 {
            if comma {
                write!(f, ",")?;
            }
            write!(f, "({}, {})", point.inner.x, point.inner.y)?;
            comma = true;
        }

        write!(f, "])")
    }
}

impl RTreeObject for Polygon {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let points: Vec<[f64; 2]> = self
            .points
            .0
            .iter()
            .map(|p| [p.inner.x, p.inner.y])
            .collect();
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
            points: PointLoop(
                coords
                    .iter()
                    .map(|d| Point {
                        inner: [d.0, d.1].into(),
                    })
                    .collect(),
            ),
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
