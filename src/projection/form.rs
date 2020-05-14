use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;

pub struct Form {
    pub polys: Vec<Polygon3>,
}

impl Form {
    pub fn new(polys: Vec<Polygon3>) -> Form {
        Form { polys }
    }

    pub fn scale(&self, scale: f64) -> Form {
        Form { polys: self.polys.iter().map(|d: &Polygon3| d.scale(scale)).collect() }
    }

    pub fn apply(&self, transformation: &dyn Transform) -> Form {
        Form { polys: self.polys.iter().map(|d: &Polygon3| d.apply(transformation)).collect() }
    }
}
