use svg::node::element::Line;
use svg::Document;
use crate::scene::Scene;

pub fn write_svg(scene: &Scene, filename: &str) {
    let bounds = scene.bounds();
        let [x1, y1] = bounds.lower();
        let [x2, y2] = bounds.upper();
        let w = x2 - x1;
        let h = y2 - y1;

        let mut doc = Document::new().set("viewBox", format!("{} {} {} {}", x1, y1, w, h));

        for line in &scene.lines {
            let svg_line = Line::new()
                .set("stroke", "black")
                .set("vector-effect", "non-scaling-stroke")
                .set("x1", line.c1.x)
                .set("y1", line.c1.y)
                .set("x2", line.c2.x)
                .set("y2", line.c2.y);

            doc = doc.add(svg_line);
        }

        svg::save(filename, &doc).unwrap();
}