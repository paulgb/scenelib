use scenelib::prelude::*;
use nalgebra::{Translation3, Rotation3};

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    
    for i in 0..50 {
        for j in 0..50 {
            let mut tet = tetrahedron()
            // Scale.
            .apply(&30.)
            // Rotate.
            .apply(&Rotation3::from_euler_angles(
                i as f64 * PI / 40.,
                j as f64 * PI / 30.,
                0.
            ))
            // Translate.
            .apply(&Translation3::from(Vec3f::new(60. * i as f64, 60. * j as f64, 0.)));

        scene3d.append(&mut tet.polys);

        }
    }

    let scene = scene3d.apply(&isometric_projection()).to_2d_scene();
    write_svg(&scene, "tetrahedron.svg");
}
