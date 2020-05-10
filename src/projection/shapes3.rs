use crate::projection::polygon3::Polygon3;
use crate::geom::types::Point3f;

const SHAPE_SCALE: f64 = 100.;

pub fn cube() -> Vec<Polygon3> {
    let p000 = Point3f::new(         0.,          0.,          0.);
    let p001 = Point3f::new(         0.,          0., SHAPE_SCALE);
    let p010 = Point3f::new(         0., SHAPE_SCALE,          0.);
    let p011 = Point3f::new(         0., SHAPE_SCALE, SHAPE_SCALE);
    let p100 = Point3f::new(SHAPE_SCALE,          0.,          0.);
    let p101 = Point3f::new(SHAPE_SCALE,          0., SHAPE_SCALE);
    let p110 = Point3f::new(SHAPE_SCALE, SHAPE_SCALE,          0.);
    let p111 = Point3f::new(SHAPE_SCALE, SHAPE_SCALE, SHAPE_SCALE);
    
    vec![
        Polygon3::new(vec![p000, p001, p011, p010]),
        Polygon3::new(vec![p100, p101, p111, p110]),

        Polygon3::new(vec![p000, p001, p101, p100]),
        Polygon3::new(vec![p010, p011, p111, p110]),

        Polygon3::new(vec![p000, p010, p110, p100]),
        Polygon3::new(vec![p001, p011, p111, p101]),
    ]
}