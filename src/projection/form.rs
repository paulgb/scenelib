use crate::projection::apply::Apply;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;

pub struct Form {
    pub polys: Vec<Polygon3>,
}

impl Form {
    pub fn new(polys: Vec<Polygon3>) -> Form {
        Form { polys }
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
