use std::iter::Sum;
use std::ops::AddAssign;

/// Represents the cost of a plot.
/// `move_cost` and `moves` are dependent on drawing order of the plot;
/// `line_cost` and `segments` are not.
#[derive(Debug, Default)]
pub struct PlotCost {
    /// The total amount of pen-down distance.
    pub move_cost: f64,
    /// The total amount of pen-up distance.
    pub line_cost: f64,
    /// The number of `LineSegments` in the plot.
    pub segments: usize,
    /// The number of pen-up moves.
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
