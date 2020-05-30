use crate::plot::Plot;
use crate::types::Point;
use std::collections::HashMap;
use svg::node::element::path::Data;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::Document;

pub trait WriteSVG {
    fn write_svg_with_fills(&self, filename: &str, layer_to_stroke: &HashMap<usize, &str>);
    fn write_svg(&self, filename: &str);
}

impl WriteSVG for Plot {
    fn write_svg(&self, filename: &str) {
        let default_fills = [(0, "black"), (1, "red"), (2, "blue")]
            .iter()
            .cloned()
            .collect();
        self.write_svg_with_fills(filename, &default_fills);
    }

    fn write_svg_with_fills(&self, filename: &str, layer_to_stroke: &HashMap<usize, &str>) {
        let Plot {
            lower_bound,
            upper_bound,
            ..
        } = self;
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

        for layer in &self.layers {
            let mut path_data = Data::new();
            let mut last: Point = Point::new(0., 0.);

            for line in &layer.lines {
                if last != line.c1 {
                    path_data = path_data.move_to((line.c1.x, line.c1.y))
                }

                path_data = path_data.line_to((line.c2.x, line.c2.y));

                last = line.c2;
            }

            let svg_line = Path::new()
                .set(
                    "stroke",
                    *layer_to_stroke.get(&layer.pen).unwrap_or(&"black"),
                )
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
