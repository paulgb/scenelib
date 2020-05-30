use crate::geom::types::Point3f;
use crate::projection::form::Form;
use crate::projection::polygon3::Polygon3;

pub fn tetrahedron() -> Form {
    let p1 = Point3f::new((8. / 9. as f64).sqrt(), 0., -1. / 3.);
    let p2 = Point3f::new(-(2. / 9. as f64).sqrt(), (2. / 3. as f64).sqrt(), -1. / 3.);
    let p3 = Point3f::new(-(2. / 9. as f64).sqrt(), -(2. / 3. as f64).sqrt(), -1. / 3.);
    let p4 = Point3f::new(0., 0., 1.);

    Form::new(vec![
        Polygon3::new(vec![p1, p2, p4]),
        Polygon3::new(vec![p2, p3, p4]),
        Polygon3::new(vec![p3, p1, p4]),
        Polygon3::new(vec![p1, p2, p3]),
    ])
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
