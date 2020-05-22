use nalgebra::{Perspective3, Translation3};
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    for i in 1..3 {
        for j in 1..3 {
            let mut cube = cube().apply(&50.)
            .apply(&Translation3::from(
                Vec3f::new(
                    i as f64 * 55.,
                    j as f64 * 55.,
                    0.,
                )
            ))
            .apply(&isometric_projection())
            .apply(&Translation3::from(Vec3f::new(
               0., 0.,
                1000.,
            )));
            scene3d.append(&mut cube.polys);
        }
    }

    let m: Perspective3<f64> = Perspective3::new(1., 3.14 / 8.0, 100.0, 1000.0);

    let scene = scene3d
        //.apply(&isometric_projection())
        .apply(&m)
        .to_2d_scene();
    write_svg(&scene, "repro_missing_line.svg");
}
