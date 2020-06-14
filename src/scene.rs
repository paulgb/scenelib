//! Representation and manipulation of 2D scenes.

use crate::draw_mode::DrawMode;
use crate::geom::line_segment::LineSegment;
use crate::geom::polygon::Polygon;
use crate::plot::{Layer, Plot};
use crate::types::Point;
use rstar::{RTree, RTreeObject, AABB};
use std::collections::BTreeMap;

/// Represents a 2D scene, indexed for efficient addition of
/// shapes. The underlying representation of a scene is a set
/// of lines, but shapes can be added as polygons. When polygons
/// are added, "fill" can be simulated by removing parts of
/// existing line segments that the polygon covers.
/// Similar to raster graphics pipelines, polygons are expected
/// to be added back-to-front.
pub struct Scene {
    lines: RTree<LineSegment>,
}

impl Scene {
    /// Constructs an empty scene.
    pub fn new() -> Scene {
        Scene {
            lines: RTree::new(),
        }
    }

    /// Converts a scene to a `Plot` without optimizing it.
    pub fn to_plot(&self) -> Plot {
        let bounds = self.lines.root().envelope();
        let lower_bound: Point = Point::from(bounds.lower());
        let upper_bound: Point = Point::from(bounds.upper());

        let mut layers_map: BTreeMap<usize, Layer> = BTreeMap::new();

        for line in &self.lines {
            let entry = layers_map
                .entry(line.pen)
                .or_insert_with(|| Layer::new(line.pen));
            entry.lines.push(line.clone());
        }

        let layers: Vec<Layer> = layers_map.values().into_iter().map(|x| x.clone()).collect();

        Plot::new(layers, lower_bound, upper_bound)
    }

    /// Computes the bounds of the scene.
    pub fn bounds(&self) -> AABB<[f64; 2]> {
        self.lines.root().envelope()
    }

    /// Adds a single line segment. Does not consider overlaps since lines have an
    /// empty area.
    pub fn add_segment(&mut self, segment: LineSegment) {
        self.lines.insert(segment);
    }

    /// Simulate "fill" for the given polygon by erasing parts of line segments
    /// which it covers. Does not outline the polygon; this should be done with
    /// `stroke_poly` *after* `fill_poly`.
    pub fn fill_poly(&mut self, poly: &Polygon) {
        let mut drop_segments: Vec<LineSegment> = Vec::new();
        let mut new_segments: Vec<LineSegment> = Vec::new();

        for line in self.lines.locate_in_envelope_intersecting(&poly.envelope()) {
            let mut crossings: Vec<f64> = Vec::new();

            let mut point_loops = vec![&poly.points];
            point_loops.extend(&poly.holes);

            let mut inter = false;
            let mut pre = false;
            let mut post = false;

            for point_loop in point_loops {
                let point_sides: Vec<(&Point, bool)> = point_loop
                    .0
                    .iter()
                    .map(|p| (p, line.point_side(p)))
                    .collect();

                let mut last = match point_sides.last() {
                    Some(p) => p,
                    None => continue,
                };

                for point_side in &point_sides {
                    if point_side.1 != last.1 {
                        // This line segment crosses.
                        let (f, _) = line
                            .intersect_lines(&LineSegment::new(*last.0, *point_side.0))
                            .unwrap();

                        if f < 0. {
                            pre = true;
                        } else if f > 1. {
                            post = true;
                        } else {
                            inter = true;
                        }

                        crossings.push(f);
                    }

                    last = point_side;
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

            let mut draw = true;
            let mut last = 0.;
            let v = line.vector();
            for frac in crossings {
                if frac > 1. {
                    break;
                } else if frac > last {
                    if draw {
                        new_segments.push(LineSegment::new_with_pen(
                            line.c1 + v * last,
                            line.c1 + v * frac,
                            line.pen,
                        ));
                    }
                    last = frac;
                }

                draw = !draw;
            }

            if draw && last < 1. {
                new_segments.push(LineSegment::new_with_pen(
                    line.c1 + v * last,
                    line.c2,
                    line.pen,
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

    /// Draw lines around the perimeter of a polygon and its holes.
    pub fn stroke_poly(&mut self, poly: &Polygon, pen: usize) {
        for line in poly.points.line_segments_with_pen(pen) {
            self.add_segment(line);
        }
        for hole in poly.holes.iter() {
            for line in hole.line_segments_with_pen(pen) {
                self.add_segment(line);
            }
        }
    }

    /// Adds a polygon to the scene with the default draw mode.
    pub fn add_poly(&mut self, poly: &Polygon) {
        self.fill_poly(&poly);
        self.stroke_poly(poly, 0);
    }

    /// Adds a polygon to the scene with a given draw mode.
    pub fn add_poly_with_draw_mode(&mut self, poly: &Polygon, draw_mode: DrawMode) {
        if draw_mode.fill {
            self.fill_poly(&poly)
        }
        if let Some(pen) = draw_mode.pen {
            self.stroke_poly(&poly, pen)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::pt;

    #[test]
    fn test_double_intersection() {
        let mut sc = Scene::new();

        let line = LineSegment::new(pt(0., 0.), pt(10., 0.));
        sc.add_segment(line);

        let poly = Polygon::new(vec![pt(10., 5.), pt(5., 0.), pt(10., -5.), pt(15., 0.)]);

        sc.fill_poly(&poly);

        let result: Vec<&LineSegment> = sc.lines.iter().collect();

        assert_eq!(vec![&LineSegment::new(pt(0., 0.), pt(5., 0.))], result);
    }

    #[test]
    fn test_double_cut() {
        let mut sc = Scene::new();

        let line = LineSegment::new(pt(0., 0.), pt(10., 0.));
        sc.add_segment(line);

        let poly = Polygon::new(vec![pt(8., 0.), pt(7., 1.), pt(8., 2.), pt(9., 1.)]);

        sc.fill_poly(&poly);

        let mut result: Vec<&LineSegment> = sc.lines.iter().collect();
        result.sort();

        assert_eq!(
            vec![
                &LineSegment::new(pt(0., 0.), pt(8., 0.)),
                &LineSegment::new(pt(8., 0.), pt(10., 0.))
            ],
            result
        );
    }

    #[test]
    fn test_basic_cases() {
        let mut sc = Scene::new();

        let untouched_line = LineSegment::new(pt(0., 9.), pt(6., 9.));
        sc.add_segment(untouched_line);

        let completely_removed_line = LineSegment::new(pt(2., 5.), pt(4., 5.));
        sc.add_segment(completely_removed_line);

        let clipped_line = LineSegment::new(pt(4., 5.), pt(4., 10.));
        sc.add_segment(clipped_line);
        let expected_clipped = LineSegment::new(pt(4., 7.), pt(4., 10.));

        let split_line = LineSegment::new(pt(0., 3.), pt(10., 3.));
        sc.add_segment(split_line);
        let expected_split1 = LineSegment::new(pt(0., 3.), pt(2., 3.));
        let expected_split2 = LineSegment::new(pt(4., 3.), pt(10., 3.));

        let poly = Polygon::new(vec![pt(3., 1.), pt(6., 7.), pt(0., 7.)]);
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
        let line1 = LineSegment::new(pt(2., 7.), pt(12., 7.));
        sc.add_segment(line1);

        let poly = Polygon::with_holes(
            vec![pt(0., 0.), pt(0., 15.), pt(15., 15.), pt(15., 0.)],
            vec![vec![pt(10., 5.), pt(10., 10.), pt(5., 10.), pt(5., 5.)]],
        );
        sc.fill_poly(&poly);

        let result: Vec<&LineSegment> = sc.lines.iter().collect();

        let expected = LineSegment::new(pt(5., 7.), pt(10., 7.));
        assert_eq!(vec![&expected], result)
    }
}
