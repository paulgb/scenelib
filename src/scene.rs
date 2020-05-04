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

    pub fn add_poly(&mut self, poly: &Polygon) {
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

            if !drop_keys.contains(i) {
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

        for line in segments {
            self.add_segment(line);
        }
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
