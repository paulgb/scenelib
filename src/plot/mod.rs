pub mod cost;

use crate::geom::line_segment::LineSegment;
use crate::optimizer::greedy_optimize;
use crate::plot::cost::PlotCost;
use crate::types::Point;

#[derive(Clone)]
pub struct Layer {
    pub lines: Vec<LineSegment>,
    pub pen: usize,
}

impl Layer {
    pub fn new(pen: usize) -> Layer {
        Layer {
            lines: Vec::new(),
            pen,
        }
    }

    pub fn cost(&self, origin: Point) -> PlotCost {
        let mut move_cost = 0.;
        let mut line_cost = 0.;
        let mut segments = 0;
        let mut moves = 0;

        let mut last = origin;

        for line in &self.lines {
            if line.c1 != last {
                move_cost += (line.c1 - last).norm();
                moves += 1;
            }
            line_cost += (line.c2 - line.c1).norm();
            last = line.c2;
            segments += 1;
        }

        move_cost += (origin - last).norm();

        PlotCost {
            move_cost,
            line_cost,
            segments,
            moves,
        }
    }
}

#[derive(Clone)]
pub struct Plot {
    pub layers: Vec<Layer>,
    pub lower_bound: Point,
    pub upper_bound: Point,
    pub origin: Point,
}

impl Plot {
    pub fn new(layers: Vec<Layer>, lower_bound: Point, upper_bound: Point) -> Plot {
        Plot {
            layers,
            lower_bound,
            upper_bound,
            origin: Point::new(0., 0.),
        }
    }

    pub fn cost(&self) -> PlotCost {
        self.layers.iter().map(|l| l.cost(self.origin)).sum()
    }

    pub fn optimize(mut self) -> Plot {
        let mut v = Vec::new();
        std::mem::swap(&mut self.layers, &mut v);
        v = v
            .into_iter()
            .map(|l| greedy_optimize(l, self.origin))
            .collect();

        Plot {
            lower_bound: self.lower_bound,
            upper_bound: self.upper_bound,
            origin: self.origin,
            layers: v,
        }
    }
}
