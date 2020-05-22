use crate::scene::Scene;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub fn write_svg(scene: &Scene, filename: &str) {
    let bounds = scene.bounds();
    let [x1, y1] = bounds.lower();
    let [x2, y2] = bounds.upper();
    let w = x2 - x1;
    let h = y2 - y1;

    let mut doc = Document::new().set("viewBox", format!("{} {} {} {}", x1, y1, w, h));
    let mut path_data = Data::new();

    for line in &scene.lines {
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
