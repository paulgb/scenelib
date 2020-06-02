use crate::plot::Plot;
use crate::scene::Scene;
use crate::types::Point;
use std::collections::HashMap;
use svg::node::element::path::Data;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::Document;

pub struct SVGWriter {
    plot: Plot,
    layer_to_stroke: HashMap<usize, String>,
}

pub trait WriteSVG {
    fn to_svg(self) -> SVGWriter;
}

fn default_fills() -> HashMap<usize, String> {
    [(0, "black".into()), (1, "red".into()), (2, "blue".into())]
        .iter()
        .cloned()
        .collect()
}

impl WriteSVG for Scene {
    fn to_svg(self) -> SVGWriter {
        SVGWriter {
            plot: self.to_plot(),
            layer_to_stroke: default_fills(),
        }
    }
}

impl WriteSVG for Plot {
    fn to_svg(self) -> SVGWriter {
        SVGWriter {
            plot: self,
            layer_to_stroke: default_fills(),
        }
    }
}

impl SVGWriter {
    pub fn save(&self, filename: &str) {
        let Plot {
            lower_bound,
            upper_bound,
            layers,
            ..
        } = &self.plot;
        let diff = upper_bound - lower_bound;
        let w = diff.x;
        let h = diff.y;
        let margin = (w * 0.05).max(h * 0.05);

        let mut doc = Document::new()
            .set(
                "viewBox",
                format!(
                    "{} {} {} {}",
                    lower_bound.x - margin,
                    lower_bound.y - margin,
                    w + margin * 2.,
                    h + margin * 2.
                ),
            )
            .set(
                "xmlns:inkscape",
                "http://www.inkscape.org/namespaces/inkscape",
            );

        for layer in layers {
            let mut path_data = Data::new();
            let mut last: Point = Point::new(0., 0.);

            for line in &layer.lines {
                if last != line.c1 {
                    path_data = path_data.move_to((line.c1.x, line.c1.y))
                }

                path_data = path_data.line_to((line.c2.x, line.c2.y));

                last = line.c2;
            }

            let d = String::from("black").clone();
            let pen_color: &str = self.layer_to_stroke.get(&layer.pen).unwrap_or(&d);
            let svg_line = Path::new()
                .set("stroke", pen_color)
                .set("fill", "none")
                .set("vector-effect", "non-scaling-stroke")
                .set("d", path_data);

            let g: Group = Group::new()
                .set("inkscape:groupmode", "layer")
                .set("inkscape:label", format!("{}", layer.pen))
                .add(svg_line);

            doc = doc.add(g);
        }

        svg::save(filename, &doc).unwrap();
    }
}
