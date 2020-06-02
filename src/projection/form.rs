use crate::draw_mode::DrawMode;
use crate::projection::apply::Apply;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;
use std::default::Default;

pub struct Form {
    pub polys: Vec<Polygon3>,
    pub draw_mode: DrawMode,
}

impl Form {
    pub fn new(polys: Vec<Polygon3>) -> Form {
        Form {
            polys,
            draw_mode: Default::default(),
        }
    }

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
