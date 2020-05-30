use std::iter::Sum;
use std::ops::AddAssign;

#[derive(Debug, Default)]
pub struct PlotCost {
    pub move_cost: f64,
    pub line_cost: f64,
    pub segments: usize,
    pub moves: usize,
}

impl AddAssign for PlotCost {
    fn add_assign(&mut self, rhs: PlotCost) {
        self.move_cost += rhs.move_cost;
        self.line_cost += rhs.line_cost;
        self.segments += rhs.segments;
        self.moves += rhs.moves;
    }
}

impl Sum for PlotCost {
    fn sum<I: Iterator<Item = PlotCost>>(iter: I) -> Self {
        let mut cost: PlotCost = Default::default();
        for c in iter {
            cost += c
        }
        cost
    }
}
