use scenelib::prelude::*;
use nalgebra::{Translation3, Rotation3};

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    
    const LAT_MIN: f64 = PI / 4.;
    const LAT_MAX: f64 = 3. * PI / 4.;
    const LAT_STEPS: u32 = 10;
    const LON_STEPS: u32 = 30;
    const RADIUS: f64 = 300.;
    const SCALE: f64 = 10.;

    for i in 0..LON_STEPS {
        for j in 0..LAT_STEPS {
            let lat = LAT_MIN + (j as f64 / LAT_STEPS as f64) * (LAT_MAX - LAT_MIN);
            let lon = (i as f64 / LON_STEPS as f64) * PI * 2.;

            let mut tet = tetrahedron()
            // Scale.
            .apply(&SCALE)
            // Translate.
            .apply(&Translation3::from(Vec3f::new(RADIUS, 0., 0.)))
            // Rotate.
            .apply(&Rotation3::from_euler_angles(
                0.,
                lon,
                lat
            ))
            // Translate.
            ;

        scene3d.append(&mut tet.polys);

        }
    }

    let scene = scene3d.to_2d_scene(&isometric_projection());
    scene.to_svg("tetrasphere.svg");
}
