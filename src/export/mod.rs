use crate::geom::types::Point2f;
use crate::plot::Plot;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub trait WriteSVG {
    fn write_svg(&self, filename: &str);
}

impl WriteSVG for Plot {
    fn write_svg(&self, filename: &str) {
        let Plot {
            lower_bound,
            upper_bound,
            ..
        } = self;
        let diff = upper_bound - lower_bound;
        let w = diff.x;
        let h = diff.y;
        let margin = (w * 0.05).max(h * 0.05);

        let mut doc = Document::new().set(
            "viewBox",
            format!("{} {} {} {}", lower_bound.x - margin, lower_bound.y - margin, w + margin * 2., h + margin * 2.),
        );
        let mut path_data = Data::new();
        let mut last: Point2f = Point2f::new(0., 0.);

        for line in &self.lines {
            if last != line.c1 {
                path_data = path_data.move_to((line.c1.x, line.c1.y))
            }

            path_data = path_data.line_to((line.c2.x, line.c2.y));

            last = line.c2;
        }

        let svg_line = Path::new()
            .set("stroke", "black")
            .set("fill", "none")
            .set("vector-effect", "non-scaling-stroke")
            .set("d", path_data);
        doc = doc.add(svg_line);

        svg::save(filename, &doc).unwrap();
    }
}
