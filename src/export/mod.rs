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

        let mut doc = Document::new().set(
            "viewBox",
            format!("{} {} {} {}", lower_bound.x, lower_bound.y, w, h),
        );
        let mut path_data = Data::new();

        for line in &self.lines {
            path_data = path_data
                .move_to((line.c1.x, line.c1.y))
                .line_to((line.c2.x, line.c2.y));
        }

        let svg_line = Path::new()
            .set("stroke", "black")
            .set("vector-effect", "non-scaling-stroke")
            .set("d", path_data);
        doc = doc.add(svg_line);

        svg::save(filename, &doc).unwrap();
    }
}
