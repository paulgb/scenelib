//! Representation of 3D shapes as a set of polygons.

use crate::draw_mode::DrawMode;
use crate::projection::apply::Apply;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;
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
