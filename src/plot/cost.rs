#[derive(Debug)]
pub struct PlotCost {
    pub move_cost: f64,
    pub line_cost: f64,
    pub segments: usize,
    pub moves: usize,
}
