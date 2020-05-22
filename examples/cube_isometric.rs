use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    
    for i in 0..40 {
        for j in 0..40 {
            let x: f64 = i as f64 * 101.;
            let y: f64 = j as f64 * 101.;
            let height = (i as f64 / 10. * PI).sin() + (j as f64 / 10. * PI).cos() + 2.;

            let mut cube = cube()
                // Scale.
                .scale3(100., 100., 10.)
                // Translate.
                .translate(x, y, 100. * height);

            scene3d.append(&mut cube.polys);
        }
    }

    let scene = scene3d.apply(&isometric_projection()).to_2d_scene();
    write_svg(&scene, "cube_isometric.svg");
}
