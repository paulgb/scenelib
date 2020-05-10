use nalgebra::Rotation3;
use scenelib::prelude::*;
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
        let sin_angle: f64 = ISO_ANGLE.cos();
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
    let mut scene3d: Vec<Polygon3> = Vec::new();

    for i in 0..40 {
        for j in 0..40 {
            let x: f64 = i as f64 * 101.;
            let y: f64 = j as f64 * 101.;
            let height = (i as f64 / 14. * PI).sin() + (j as f64 / 20. * PI).cos() + 2.;

            let mut cube = cube()
                .scale(100.)
                .apply(&|&d| Point3f::new(d.x, d.y, (d.z - 50.) * height) + Vec3f::new(x, y, 0.));

            scene3d.append(&mut cube.polys);
        }
    }

    let proj = SimpleProj {};

    let mut dist_scene: Vec<(f64, Polygon3)> = scene3d
        .into_iter()
        .map(|p| (poly_distance(&p, &proj), p))
        .collect();

    let mut scene = Scene::new();

    dist_scene.sort_by(|a, b| (b.0).partial_cmp(&(a.0)).unwrap());

    for (_, poly3) in dist_scene {
        let pp = poly3.project_to_poly(&proj);
        scene.add_poly(&pp);
        //scene.stroke_poly(&pp);
    }

    scene.to_svg("cube_scale.svg");
}
