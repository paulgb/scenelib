pub mod cost;

use crate::geom::line_segment::LineSegment;
use crate::geom::types::Point;
use crate::plot::cost::PlotCost;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Plot {
    #[wasm_bindgen(skip)]
    pub lines: Vec<LineSegment>,
    #[wasm_bindgen(skip)]
    pub lower_bound: Point,
    #[wasm_bindgen(skip)]
    pub upper_bound: Point,
    #[wasm_bindgen(skip)]
    pub origin: Point,
}

impl Plot {
    pub fn new(lines: Vec<LineSegment>, lower_bound: Point, upper_bound: Point) -> Plot {
        Plot {
            lines,
            lower_bound,
            upper_bound,
            origin: Point {inner: [0., 0.].into()}
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
                move_cost += (line.c1.inner - last.inner).norm();
                moves += 1;
            }
            line_cost += (line.c2.inner - line.c1.inner).norm();
            last = line.c2;
            segments += 1;
        }

        move_cost += (self.origin.inner - last.inner).norm();

        PlotCost {
            move_cost,
            line_cost,
            segments,
            moves
        }
    }
}
