//! Implements some basic Platonic solids.
//! Currently only two are implemented (PRs accepted for the others).

use crate::projection::form::Form;
use crate::projection::polygon3::Polygon3;
use crate::projection::types3::Point3;

/// Construct a tetrahedron `Form`.
pub fn tetrahedron() -> Form {
    let p1 = Point3::new((8. / 9. as f64).sqrt(), 0., -1. / 3.);
    let p2 = Point3::new(-(2. / 9. as f64).sqrt(), (2. / 3. as f64).sqrt(), -1. / 3.);
    let p3 = Point3::new(-(2. / 9. as f64).sqrt(), -(2. / 3. as f64).sqrt(), -1. / 3.);
    let p4 = Point3::new(0., 0., 1.);
    let origin = Point3::new(0., 0., 0.);

    Form::new(vec![
        Polygon3::new(vec![p1, p2, p4], (p1+(p2-origin)+(p4-origin))/3.),
        Polygon3::new(vec![p2, p3, p4], (p2+(p3-origin)+(p4-origin))/3.),
        Polygon3::new(vec![p3, p1, p4], (p3+(p1-origin)+(p4-origin))/3.),
        Polygon3::new(vec![p1, p2, p3], (p1+(p2-origin)+(p3-origin))/3.),
    ])
}

/// Construct a cube `Form`.
pub fn cube() -> Form {
    let p000 = Point3::new(-0.5, -0.5, -0.5);
    let p001 = Point3::new(-0.5, -0.5, 0.5);
    let p010 = Point3::new(-0.5, 0.5, -0.5);
    let p011 = Point3::new(-0.5, 0.5, 0.5);
    let p100 = Point3::new(0.5, -0.5, -0.5);
    let p101 = Point3::new(0.5, -0.5, 0.5);
    let p110 = Point3::new(0.5, 0.5, -0.5);
    let p111 = Point3::new(0.5, 0.5, 0.5);

    Form::new(vec![
        Polygon3::new(vec![p000, p001, p011, p010], Point3::new(-0.5, 0., 0.)),
        Polygon3::new(vec![p100, p101, p111, p110], Point3::new(0.5, 0., 0.)),
        Polygon3::new(vec![p000, p001, p101, p100], Point3::new(0., -0.5, 0.)),
        Polygon3::new(vec![p010, p011, p111, p110], Point3::new(0., 0.5, 0.)),
        Polygon3::new(vec![p000, p010, p110, p100], Point3::new(0., 0., -0.5)),
        Polygon3::new(vec![p001, p011, p111, p101], Point3::new(0., 0., 0.5)),
    ])
}
