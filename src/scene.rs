use crate::geom::line_segment::LineSegment;
use crate::geom::polygon::Polygon;
use std::collections::{HashMap, HashSet};
use svg::node::element::Line;
use svg::Document;

pub struct Scene {
    lines: HashMap<usize, LineSegment>,
    next_index: usize,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            lines: HashMap::new(),
            next_index: 0,
        }
    }

    pub fn add_segment(&mut self, segment: LineSegment) {
        self.lines.insert(self.next_index, segment);
        self.next_index += 1;
    }

    pub fn fill_poly(&mut self, poly: &Polygon) {
        let segments = poly.line_segments();
        let mut drop_keys: HashSet<usize> = HashSet::new();
        let mut new_segments: Vec<LineSegment> = Vec::new();

        for (i, line) in &self.lines {
            let mut crossings: Vec<f64> = Vec::new();
            for poly_line in &segments {
                if let Some(frac) = line.intersect_segment(poly_line) {
                    if frac < 1. {
                        crossings.push(frac);
                    }
                    drop_keys.insert(*i);
                }
            }

            // if all crossings are < 0 and there are an even number of crossings,
            // continue.
            if crossings.iter().all(|x| *x < 0.0) && (crossings.len() % 2 == 0) {
                continue;
            }

            crossings.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut last = 0.0;
            let mut draw = true;
            let v = line.vector();
            for frac in crossings {
                if frac >= last {
                    if draw {
                        new_segments.push(LineSegment::new(line.c1 + v * last, line.c1 + v * frac));
                    }

                    last = frac;
                }
                draw = !draw;
            }

            if draw {
                new_segments.push(
                    LineSegment::new(line.c1 + v * last, line.c2)
                );
            }  
        }

        for i in drop_keys {
            self.lines.remove(&i);
        }

        for line in new_segments {
            self.add_segment(line);
        }
    }

    pub fn stroke_poly(&mut self, poly: &Polygon) {
        for line in poly.line_segments() {
            self.add_segment(line);
        }
    }

    pub fn add_poly(&mut self, poly: &Polygon) {
        self.fill_poly(poly);
        self.stroke_poly(poly);
    }

    pub fn to_svg(&self, filename: &str) {
        let mut doc = Document::new();

        for (i, line) in &self.lines {
            let svg_line = Line::new()
                .set("id", format!("{}", i))
                .set("stroke", "black")
                .set("x1", line.c1.x)
                .set("y1", line.c1.y)
                .set("x2", line.c2.x)
                .set("y2", line.c2.y);

            doc = doc.add(svg_line);
        }

        svg::save(filename, &doc).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::coord::Coord;

    #[test]
    fn test_complete_removal() {

    }

    #[test]
    fn test_no_overlap() {
        let mut sc = Scene::new();

        let untouched_line = LineSegment::new(
            Coord::new(0.,9.), 
            Coord::new(6., 9.)
        );
        sc.add_segment(untouched_line);

        let completely_removed_line = LineSegment::new(
            Coord::new(2.,5.), 
            Coord::new(4., 5.)
        );
        sc.add_segment(completely_removed_line);

        let clipped_line = LineSegment::new(
            Coord::new(4., 5.),
            Coord::new(4., 10.)
        );
        sc.add_segment(clipped_line);
        let expected_clipped = LineSegment::new(
            Coord::new(4., 7.),
            Coord::new(4., 10.)
        );

        let split_line = LineSegment::new(
            Coord::new(0., 3.),
            Coord::new(10., 3.)
        );
        sc.add_segment(split_line);
        let expected_split1 = LineSegment::new(
            Coord::new(0., 3.),
            Coord::new(2., 3.)
        );
        let expected_split2 = LineSegment::new(
            Coord::new(4., 3.),
            Coord::new(10., 3.)
        );

        let poly = Polygon::new(vec![
            Coord::new(3., 1.),
            Coord::new(6., 7.),
            Coord::new(0., 7.),
        ]);
        sc.fill_poly(&poly);
        let mut result: Vec<&LineSegment> = sc.lines.values().collect();
        result.sort();

        let mut expect = vec![
            &untouched_line,
            &expected_clipped,
            &expected_split2,
            &expected_split1,
        ];
        expect.sort();

        assert_eq!(
            expect,
            result
        )

    }
}