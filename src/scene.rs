use crate::types::line_segment::LineSegment;
use crate::types::polygon::Polygon;
use crate::types::point::Point;
use crate::plot::Plot;
use rstar::{RTree, RTreeObject, AABB};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Scene {
    lines: RTree<LineSegment>,
}

#[wasm_bindgen]
impl Scene {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Scene {
        Scene {
            lines: RTree::new(),
        }
    }

    pub fn to_plot(&self) -> Plot {
        let bounds = self.lines.root().envelope();
        let lower_bound: Point = Point {
            inner: bounds.lower().into(),
        };
        let upper_bound: Point = Point {
            inner: bounds.upper().into(),
        };
        let lines: Vec<LineSegment> = self.lines.iter().map(|d| *d).collect();

        Plot::new(lines, lower_bound, upper_bound)
    }
}

impl Scene {
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
            let v = line.vector().inner;
            let mut last_direction: Option<bool> = None;
            let mut draw = true;
            for (frac, direction) in crossings {
                if frac > 1.0 {
                    break;
                } else if frac >= last {
                    if draw {
                        new_segments.push(LineSegment::new(
                            Point {
                                inner: line.c1.inner + v * last,
                            },
                            Point {
                                inner: line.c1.inner + v * frac,
                            },
                        ));
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
                new_segments.push(LineSegment::new(
                    Point {
                        inner: line.c1.inner + v * last,
                    },
                    line.c2,
                ));
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
    //use crate::geom::types::Point;

    #[test]
    fn test_double_intersection() {
        let mut sc = Scene::new();

        let line = LineSegment::new(
            Point {
                inner: [0., 0.].into(),
            },
            Point {
                inner: [10., 0.].into(),
            },
        );
        sc.add_segment(line);

        let poly = Polygon::new(vec![
            Point {
                inner: [10., 5.].into(),
            },
            Point {
                inner: [5., 0.].into(),
            },
            Point {
                inner: [10., -5.].into(),
            },
            Point {
                inner: [15., 0.].into(),
            },
        ]);

        sc.fill_poly(&poly);

        let result: Vec<&LineSegment> = sc.lines.iter().collect();

        assert_eq!(
            vec![&LineSegment::new(
                Point {
                    inner: [0., 0.].into()
                },
                Point {
                    inner: [5., 0.].into()
                }
            )],
            result
        );
    }

    #[test]
    fn test_double_cut() {
        let mut sc = Scene::new();

        let line = LineSegment::new(
            Point {
                inner: [0., 0.].into(),
            },
            Point {
                inner: [10., 0.].into(),
            },
        );
        sc.add_segment(line);

        let poly = Polygon::new(vec![
            Point {
                inner: [8., 0.].into(),
            },
            Point {
                inner: [7., 1.].into(),
            },
            Point {
                inner: [8., 2.].into(),
            },
            Point {
                inner: [9., 1.].into(),
            },
        ]);

        sc.fill_poly(&poly);

        let mut result: Vec<&LineSegment> = sc.lines.iter().collect();
        result.sort();

        assert_eq!(
            vec![
                &LineSegment::new(
                    Point {
                        inner: [0., 0.].into()
                    },
                    Point {
                        inner: [8., 0.].into()
                    }
                ),
                &LineSegment::new(
                    Point {
                        inner: [8., 0.].into()
                    },
                    Point {
                        inner: [10., 0.].into()
                    }
                )
            ],
            result
        );
    }

    #[test]
    fn test_basic_cases() {
        let mut sc = Scene::new();

        let untouched_line = LineSegment::new(
            Point {
                inner: [0., 9.].into(),
            },
            Point {
                inner: [6., 9.].into(),
            },
        );
        sc.add_segment(untouched_line);

        let completely_removed_line = LineSegment::new(
            Point {
                inner: [2., 5.].into(),
            },
            Point {
                inner: [4., 5.].into(),
            },
        );
        sc.add_segment(completely_removed_line);

        let clipped_line = LineSegment::new(
            Point {
                inner: [4., 5.].into(),
            },
            Point {
                inner: [4., 10.].into(),
            },
        );
        sc.add_segment(clipped_line);
        let expected_clipped = LineSegment::new(
            Point {
                inner: [4., 7.].into(),
            },
            Point {
                inner: [4., 10.].into(),
            },
        );

        let split_line = LineSegment::new(
            Point {
                inner: [0., 3.].into(),
            },
            Point {
                inner: [10., 3.].into(),
            },
        );
        sc.add_segment(split_line);
        let expected_split1 = LineSegment::new(
            Point {
                inner: [0., 3.].into(),
            },
            Point {
                inner: [2., 3.].into(),
            },
        );
        let expected_split2 = LineSegment::new(
            Point {
                inner: [4., 3.].into(),
            },
            Point {
                inner: [10., 3.].into(),
            },
        );

        let poly = Polygon::new(vec![
            Point {
                inner: [3., 1.].into(),
            },
            Point {
                inner: [6., 7.].into(),
            },
            Point {
                inner: [0., 7.].into(),
            },
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
        let line1 = LineSegment::new(
            Point {
                inner: [2., 7.].into(),
            },
            Point {
                inner: [12., 7.].into(),
            },
        );
        sc.add_segment(line1);

        let poly = Polygon::with_holes(
            vec![
                Point {
                    inner: [0., 0.].into(),
                },
                Point {
                    inner: [0., 15.].into(),
                },
                Point {
                    inner: [15., 15.].into(),
                },
                Point {
                    inner: [15., 0.].into(),
                },
            ],
            vec![vec![
                Point {
                    inner: [10., 5.].into(),
                },
                Point {
                    inner: [10., 10.].into(),
                },
                Point {
                    inner: [5., 10.].into(),
                },
                Point {
                    inner: [5., 5.].into(),
                },
            ]],
        );
        sc.fill_poly(&poly);

        let result: Vec<&LineSegment> = sc.lines.iter().collect();

        let expected = LineSegment::new(
            Point {
                inner: [5., 7.].into(),
            },
            Point {
                inner: [10., 7.].into(),
            },
        );
        assert_eq!(vec![&expected], result)
    }
}
