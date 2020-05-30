use crate::plot::Plot;
use crate::types::point::Point;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::Value;
use svg::Document;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn to_svg_path(plot: &Plot) -> String {
    let mut path_data = Data::new();
    let mut last: Point = Point {
        inner: [0., 0.].into(),
    };

    for line in &plot.lines {
        if last != line.c1 {
            path_data = path_data.move_to((line.c1.inner.x, line.c1.inner.y))
        }

        path_data = path_data.line_to((line.c2.inner.x, line.c2.inner.y));

        last = line.c2;
    }
    Value::from(path_data).to_string()
}

#[wasm_bindgen]
pub fn write_svg(plot: &Plot, filename: &str) {
    let Plot {
        lower_bound,
        upper_bound,
        ..
    } = plot;
    let diff = upper_bound.inner - lower_bound.inner;
    let w = diff.x;
    let h = diff.y;
    let margin = (w * 0.05).max(h * 0.05);

    let mut doc = Document::new().set(
        "viewBox",
        format!(
            "{} {} {} {}",
            lower_bound.inner.x - margin,
            lower_bound.inner.y - margin,
            w + margin * 2.,
            h + margin * 2.
        ),
    );

    let path_data = to_svg_path(plot);
    let svg_line = Path::new()
        .set("stroke", "black")
        .set("fill", "none")
        .set("vector-effect", "non-scaling-stroke")
        .set("d", path_data);
    doc = doc.add(svg_line);

    svg::save(filename, &doc).unwrap();
}
