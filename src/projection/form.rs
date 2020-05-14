use crate::geom::types::Point3f;
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

pub fn cube() -> Form {
    let p000 = Point3f::new(-0.5, -0.5, -0.5);
    let p001 = Point3f::new(-0.5, -0.5, 0.5);
    let p010 = Point3f::new(-0.5, 0.5, -0.5);
    let p011 = Point3f::new(-0.5, 0.5, 0.5);
    let p100 = Point3f::new(0.5, -0.5, -0.5);
    let p101 = Point3f::new(0.5, -0.5, 0.5);
    let p110 = Point3f::new(0.5, 0.5, -0.5);
    let p111 = Point3f::new(0.5, 0.5, 0.5);

    Form::new(vec![
        Polygon3::new(vec![p000, p001, p011, p010]),
        Polygon3::new(vec![p100, p101, p111, p110]),
        Polygon3::new(vec![p000, p001, p101, p100]),
        Polygon3::new(vec![p010, p011, p111, p110]),
        Polygon3::new(vec![p000, p010, p110, p100]),
        Polygon3::new(vec![p001, p011, p111, p101]),
    ])
}
