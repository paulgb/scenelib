//! Representation of 3D shapes as a set of polygons.

use crate::draw_mode::DrawMode;
use crate::geom::polygon::Polygon;
use crate::projection::apply::Apply;
use crate::projection::apply::ApplyOps;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;
use crate::projection::types3::Point3;
use crate::projection::types3::Vector3;
use crate::types::Point;
use std::default::Default;

/// Represents a 3D shape.
pub struct Form {
    /// Polygons that make up this shape.
    pub polys: Vec<Polygon3>,
    /// Mode of drawing this shape.
    pub draw_mode: DrawMode,
}

impl Form {
    /// Construct a form from a list of polygons using the default drawing mode.
    pub fn new(polys: Vec<Polygon3>) -> Form {
        Form {
            polys,
            draw_mode: Default::default(),
        }
    }

    /// Create a form by extruding a polygon for a given height.
    /// The resulting shape has the same X and Y values with Z values
    /// taken from +/- the given height.
    /// Note that this means the actual height of the shape is 2x the
    /// given height. This is analogous to the way that a circle has
    /// a diameter 2x the radius.
    pub fn extrude_from_poly(poly: &Polygon, height: f64) -> Form {
        let mut polys: Vec<Polygon3> = Vec::new();

        let cap = Polygon3 {
            points: poly
                .points
                .0
                .iter()
                .map(|p: &Point| Point3::new(p.x, p.y, 0.0))
                .collect(),
        };

        polys.push(cap.clone().translate(Vector3::new(0., 0., height)));
        polys.push(cap.translate(Vector3::new(0., 0., -height)));

        for line in poly.points.line_segments() {
            let p = Polygon3 {
                points: vec![
                    Point3::new(line.c1.x, line.c1.y, height),
                    Point3::new(line.c2.x, line.c2.y, height),
                    Point3::new(line.c2.x, line.c2.y, -height),
                    Point3::new(line.c1.x, line.c1.y, -height),
                ],
            };

            polys.push(p);
        }

        Form {
            polys,
            draw_mode: Default::default(),
        }
    }

    /// Set the drawing mode of this form.
    pub fn draw_mode(mut self, draw_mode: DrawMode) -> Form {
        self.draw_mode = draw_mode;
        self
    }
}

impl Apply for Form {
    fn apply(mut self, transformation: &dyn Transform) -> Form {
        self.polys = self
            .polys
            .into_iter()
            .map(|d| d.apply(transformation))
            .collect();
        self
    }
}
