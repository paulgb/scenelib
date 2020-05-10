use scenelib::prelude::*;
//use nalgebra::{Orthographic3};

struct SimpleProj {}

const ISO_ANGLE: f64 = PI / 9.;

impl Projection for SimpleProj {
    fn project(&self, point: &Point3f) -> Point2f {
        let sin_angle: f64 = ISO_ANGLE.sin();
        let cos_angle: f64 = ISO_ANGLE.cos();

        Point2f::new(
            point.x * -cos_angle + point.y * cos_angle,
            point.z + sin_angle * point.x + sin_angle * point.y,
        )
    }

    fn distance(&self, point: &Point3f) -> f64 {
        let sin_angle: f64 = ISO_ANGLE.sin();
        let cos_angle: f64 = ISO_ANGLE.cos();
        // TODO: I made this up
        -(point.x * sin_angle + point.y * sin_angle - point.z)
    }
}

fn poly_distance(poly: &Polygon3, proj: &dyn Projection) -> f64 {
    let mut sum = 0.;
    let mut count = 0.;

    for point in &poly.points {
        count += 1.;
        sum += proj.distance(&point)
    }

    return sum / count;
}

fn main() {
    let mut scene = Scene::new();
    let cube = cube();

    let proj = SimpleProj {};

    let mut dist_cube: Vec<(f64, Polygon3)> = cube.into_iter().map(|p| (poly_distance(&p, &proj), p)).collect();

    dist_cube.sort_by(|a, b| (b.0).partial_cmp(&(a.0)).unwrap());

    for (dist, poly3) in dist_cube {
        let pp = poly3.project_to_poly(&proj);
        scene.add_poly(&pp);
    }
    
    scene.to_svg("cube.svg");
}
