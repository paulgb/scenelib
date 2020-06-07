//! Tools for exporting plots. Currently supports SVG.

use crate::plot::Plot;
use crate::scene::Scene;
use crate::types::Point;
use std::collections::HashMap;
use svg::node::element::path::Data;
use svg::node::element::Group;
use svg::node::element::Path;
use svg::Document;

const DEFAULT_WIDTH: f64 = 300.;
const DEFAULT_HEIGHT: f64 = 218.;

/// Builder for writing a `Plot` to an `.svg` file.
pub struct SVGWriter {
    /// The plot to write.
    plot: Plot,
    /// A map from pen number to SVG color. This is used only to set the color
    /// used for screen display, and does not affect the final plot.
    layer_to_stroke: HashMap<usize, String>,
    /// Fraction of the critical dimension to use; values less than 1 provide a
    /// margin around the image.
    fill_fraction: f64,
    /// Width of the output in mm.
    width: f64,
    /// Height of the output in mm.
    height: f64,
}

/// Objects that can be turned into an SVG builder.
pub trait WriteSVG {
    /// Turn this object into a builder for writing an `.svg` file.
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
        self.to_plot().optimize().to_svg()
    }
}

impl WriteSVG for Plot {
    fn to_svg(self) -> SVGWriter {
        SVGWriter {
            plot: self,
            layer_to_stroke: default_fills(),
            fill_fraction: 0.9,
            height: DEFAULT_HEIGHT,
            width: DEFAULT_WIDTH,
        }
    }
}

impl SVGWriter {
    pub fn axidraw_portrait(&mut self) -> &mut SVGWriter {
        self.width = DEFAULT_HEIGHT;
        self.height = DEFAULT_WIDTH;
        self
    }

    /// Write the resulting SVG to a file at the given location.
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
        
        let scale = (self.width / w).min(self.height / h) * self.fill_fraction;
        let x_offset = (self.width - (scale * w)) / 2. - lower_bound.x * scale;
        let y_offset = (self.height - (scale * h)) / 2. - lower_bound.y * scale;

        let scale_point = |p: Point| (
            p.x * scale + x_offset,
            p.y * scale + y_offset
        );

        let mut doc = Document::new()
            .set(
                "height",
                format!("{}mm", self.height)
            )
            .set(
                "width",
                format!("{}mm", self.width)
            )
            .set(
                "viewBox",
                format!(
                    "{} {} {} {}",
                    0,
                    0,
                    self.width,
                    self.height
                ),
            )
            .set(
                "xmlns:inkscape",
                "http://www.inkscape.org/namespaces/inkscape",
            );

        for layer in layers {
            let mut path_data = Data::new();
            let mut last: Option<Point> = None;

            for line in &layer.lines {
                if last != Some(line.c1) {
                    path_data = path_data.move_to(scale_point(line.c1))
                }

                path_data = path_data.line_to(scale_point(line.c2));

                last = Some(line.c2);
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
