use crate::geom::line_segment::LineSegment;
use crate::geom::polygon::Polygon;
use rstar::{RTree, RTreeObject, AABB};
use crate::plot::Plot;
use crate::geom::types::Point2f;
pub struct Scene {
    lines: RTree<LineSegment>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            lines: RTree::new(),
        }
    }

    pub fn to_plot(&self) -> Plot {
        let bounds = self.lines.root().envelope();
        let lower_bound: Point2f = Point2f::from(bounds.lower());
        let upper_bound: Point2f = Point2f::from(bounds.upper());
        let lines = self.lines.iter().map(|d| *d).collect();

        Plot::new(lines, lower_bound, upper_bound)
    }

    pub fn bounds(&self) -> AABB<[f64; 2]> {
        self.lines.root().envelope()
    }

    pub fn add_segment(&mut self, segment: LineSegment) {
        self.lines.insert(segment);
    }

    pub fn fill_poly(&mut self, poly: &Polygon) {
        let segments = {
            let mut s = poly.points.line_segments();

            for hole in &poly.holes {
                s.append(&mut hole.line_segments().clone());
            }

            s
        };

        let mut drop_segments: Vec<LineSegment> = Vec::new();
        let mut new_segments: Vec<LineSegment> = Vec::new();

        for line in self.lines.locate_in_envelope_intersecting(&poly.envelope()) {
            let mut crossings: Vec<(f64, bool)> = Vec::new();
            let mut pre = false;
            let mut post = false;
            let mut inter = false;
            for poly_line in &segments {
                if let Some((frac, direction)) = line.intersect_segment(poly_line) {
                    if frac < 0. {
                        pre = true;
                    } else if frac > 1. {
                        post = true;
                    } else {
                        inter = true;
                    }
                    crossings.push((frac, direction));
                }
            }

            if !inter && (!pre || !post) {
                // If the line does not have any intersections, we do not need to remove it.
                // This is not necessary for correctness (since the segment would be removed
                // and added back), but it's a big performance improvement.
                continue;
            }

            drop_segments.push(*line);

            crossings.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut last = 0.0;
            let v = line.vector();
            let mut last_direction: Option<bool> = None;
            let mut draw = true;
            for (frac, direction) in crossings {
                if frac > 1.0 {
                    break;
                } else if frac >= last {
                    if draw {
                        new_segments.push(LineSegment::new(line.c1 + v * last, line.c1 + v * frac));
                    }

                    last = frac;
                }

                if let Some(ld) = last_direction {
                    if ld != direction {
                        // Only flip draw if the direction has actually flipped.
                        draw = !draw;
                    }
                } else {
                    draw = !draw;
                }
                last_direction = Some(direction)
            }

            if draw {
                new_segments.push(LineSegment::new(line.c1 + v * last, line.c2));
            }
        }

        for seg in drop_segments {
            self.lines.remove(&seg);
        }

        for line in new_segments {
            self.add_segment(line);
        }
    }

    pub fn stroke_poly(&mut self, poly: &Polygon) {
        for line in poly.points.line_segments() {
            self.add_segment(line);
        }
        for hole in poly.holes.iter() {
            for line in hole.line_segments() {
                self.add_segment(line);
            }
        }
    }

    pub fn add_poly(&mut self, poly: &Polygon) {
        self.fill_poly(poly);
        self.stroke_poly(poly);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::types::Point2f;

    #[test]
    fn test_double_intersection() {
        let mut sc = Scene::new();

        let line = LineSegment::new(Point2f::new(0., 0.), Point2f::new(10., 0.));
        sc.add_segment(line);

        let poly = Polygon::new(vec![
            Point2f::new(10., 5.),
            Point2f::new(5., 0.),
            Point2f::new(10., -5.),
            Point2f::new(15., 0.),
        ]);

        sc.fill_poly(&poly);

        let result: Vec<&LineSegment> = sc.lines.iter().collect();

        assert_eq!(
            vec![&LineSegment::new(
                Point2f::new(0., 0.),
                Point2f::new(5., 0.)
            )],
            result
        );
    }

    #[test]
    fn test_double_cut() {
        let mut sc = Scene::new();

        let line = LineSegment::new(Point2f::new(0., 0.), Point2f::new(10., 0.));
        sc.add_segment(line);

        let poly = Polygon::new(vec![
            Point2f::new(8., 0.),
            Point2f::new(7., 1.),
            Point2f::new(8., 2.),
            Point2f::new(9., 1.),
        ]);

        sc.fill_poly(&poly);

        let mut result: Vec<&LineSegment> = sc.lines.iter().collect();
        result.sort();

        assert_eq!(
            vec![
                &LineSegment::new(Point2f::new(0., 0.), Point2f::new(8., 0.)),
                &LineSegment::new(Point2f::new(8., 0.), Point2f::new(10., 0.))
            ],
            result
        );
    }

    #[test]
    fn test_basic_cases() {
        let mut sc = Scene::new();

        let untouched_line = LineSegment::new(Point2f::new(0., 9.), Point2f::new(6., 9.));
        sc.add_segment(untouched_line);

        let completely_removed_line = LineSegment::new(Point2f::new(2., 5.), Point2f::new(4., 5.));
        sc.add_segment(completely_removed_line);

        let clipped_line = LineSegment::new(Point2f::new(4., 5.), Point2f::new(4., 10.));
        sc.add_segment(clipped_line);
        let expected_clipped = LineSegment::new(Point2f::new(4., 7.), Point2f::new(4., 10.));

        let split_line = LineSegment::new(Point2f::new(0., 3.), Point2f::new(10., 3.));
        sc.add_segment(split_line);
        let expected_split1 = LineSegment::new(Point2f::new(0., 3.), Point2f::new(2., 3.));
        let expected_split2 = LineSegment::new(Point2f::new(4., 3.), Point2f::new(10., 3.));

        let poly = Polygon::new(vec![
            Point2f::new(3., 1.),
            Point2f::new(6., 7.),
            Point2f::new(0., 7.),
        ]);
        sc.fill_poly(&poly);
        let mut result: Vec<&LineSegment> = sc.lines.iter().collect();
        result.sort();

        let mut expect = vec![
            &untouched_line,
            &expected_clipped,
            &expected_split1,
            &expected_split2,
        ];
        expect.sort();
        result.sort();

        assert_eq!(expect, result)
    }

    #[test]
    fn test_polygon_hole() {
        let mut sc = Scene::new();
        let line1 = LineSegment::new(Point2f::new(2., 7.), Point2f::new(12., 7.));
        sc.add_segment(line1);

        let poly = Polygon::with_holes(
            vec![
                Point2f::new(0., 0.),
                Point2f::new(0., 15.),
                Point2f::new(15., 15.),
                Point2f::new(15., 0.),
            ],
            vec![vec![
                Point2f::new(10., 5.),
                Point2f::new(10., 10.),
                Point2f::new(5., 10.),
                Point2f::new(5., 5.),
                ]],
        );
        sc.fill_poly(&poly);

        let result: Vec<&LineSegment> = sc.lines.iter().collect();

        let expected = LineSegment::new(
            Point2f::new(5., 7.),
            Point2f::new(10., 7.)
        );
        assert_eq!(vec![
            &expected
        ], result)
    }
}
