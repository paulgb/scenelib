pub mod cost;

use crate::geom::line_segment::LineSegment;
use crate::geom::types::Point2f;
use crate::plot::cost::PlotCost;

pub struct Plot {
    pub lines: Vec<LineSegment>,
    pub lower_bound: Point2f,
    pub upper_bound: Point2f,
    pub origin: Point2f,
}

impl Plot {
    pub fn new(lines: Vec<LineSegment>, lower_bound: Point2f, upper_bound: Point2f) -> Plot {
        Plot {
            lines,
            lower_bound,
            upper_bound,
            origin: Point2f::new(0., 0.)
        }
    }

    pub fn cost(&self) -> PlotCost {
        let mut move_cost = 0.;
        let mut line_cost = 0.;
        let mut segments = 0;
        let mut moves = 0;

        let mut last = self.origin.clone();

        for line in &self.lines {
            if line.c1 != last {
                move_cost += (line.c1 - last).norm();
                moves += 1;
            }
            line_cost += (line.c2 - line.c1).norm();
            last = line.c2;
            segments += 1;
        }

        move_cost += (self.origin - last).norm();

        PlotCost {
            move_cost,
            line_cost,
            segments,
            moves
        }
    }
}
