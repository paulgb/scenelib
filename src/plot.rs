use crate::geom::line_segment::LineSegment;
use crate::geom::types::Point2f;

pub struct Plot {
    pub lines: Vec<LineSegment>,
    pub lower_bound: Point2f,
    pub upper_bound: Point2f,
}

impl Plot {
    pub fn new(lines: Vec<LineSegment>, lower_bound: Point2f, upper_bound: Point2f) -> Plot {
        Plot {
            lines,
            lower_bound,
            upper_bound,
        }
    }
}
