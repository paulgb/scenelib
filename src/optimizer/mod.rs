use crate::geom::line_segment::LineSegment;
use crate::types::Point;
use crate::plot::Plot;
use rstar::{PointDistance, RTree, RTreeObject, AABB};

#[derive(PartialEq, Clone)]
struct TreeElement {
    start: Point,
    end: Point,
}

impl TreeElement {
    pub fn swap(&self) -> TreeElement {
        TreeElement {
            start: self.end,
            end: self.start,
        }
    }
}

impl RTreeObject for TreeElement {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.start.x, self.start.y], [self.start.x, self.start.y])
    }
}

impl PointDistance for TreeElement {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        (self.start - Point::from(*point)).norm()
    }
}

pub fn greedy_optimize(plot: Plot) -> Plot {
    let points: Vec<TreeElement> = plot
        .lines
        .iter()
        .flat_map(|d| {
            vec![
                TreeElement {
                    start: d.c1,
                    end: d.c2,
                },
                TreeElement {
                    start: d.c2,
                    end: d.c1,
                },
            ]
        })
        .collect();

    let mut tree = RTree::bulk_load(points);
    let mut lines: Vec<LineSegment> = Vec::new();
    let mut cursor = plot.origin;

    while tree.size() > 0 {
        let next = (*tree.nearest_neighbor(&[cursor.x, cursor.y]).unwrap()).clone();

        lines.push(LineSegment::new(next.start, next.end));
        cursor = next.end;

        tree.remove(&next);
        tree.remove(&next.swap());
    }

    Plot::new(lines, plot.lower_bound, plot.upper_bound)
}
