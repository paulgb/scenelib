use scenelib::prelude::*;
use nalgebra::{Perspective3, Translation3};


fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    
    for i in 0..40 {
        for j in 0..40 {
            let x: f64 = i as f64 * 101.;
            let y: f64 = j as f64 * 101.;
            let height = (i as f64 / 10. * PI).sin() + (j as f64 / 10. * PI).cos() + 2.;

            let mut cube = cube()
                // Scale.
                .apply(&Vec3f::new(100., 100., 10.))
                // Translate.
                .apply(&Translation3::from(Vec3f::new(x, y, 100. * height)));

            scene3d.append(&mut cube.polys);
        }
    }
    let m: Perspective3<f64> = Perspective3::new(1., 3.14 / 8.0, 100.0, 1000.0);
    let scene = scene3d
        .apply(&isometric_projection())
        .apply(&(1. / 15.))
        
        .apply(&Translation3::from(Vec3f::new(
            0., 0.,
             1000.,
         )))
         .apply(&m)
         
        .to_2d_scene();
    write_svg(&scene, "cube_isometric_proj.svg");
}
