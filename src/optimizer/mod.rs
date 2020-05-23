use crate::plot::Plot;
use crate::geom::types::Point2f;

pub type CostPair = (f64, f64);

pub fn scene_cost(plot: &Plot, origin: &Point2f) -> CostPair {
    let mut move_dist = 0.;
    let mut line_dist = 0.;

    let mut last = origin.clone();

    for line in &plot.lines {
        move_dist += (line.c1 - last).norm();
        line_dist += (line.c2 - line.c1).norm();
        last = line.c2;
    }

    move_dist += (origin - last).norm();

    (move_dist, line_dist)
}
