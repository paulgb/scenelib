use nalgebra::Perspective3;
use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();
    let mut cube = cube()
        .scale3(100., 100., 10.)
        .translate(0., 0., 0.);

    scene3d.append(&mut cube.polys);
    
    let m: Perspective3<f64> = Perspective3::new(1., 3.14 / 8.0, 100.0, 1000.0);
    scene3d = scene3d
        .apply(&isometric_projection())
        .scale(1. / 15.)
        .translate(0., 0., 1000.)
        .apply(&m);
    
    for poly in scene3d.project() {
        println!("{:?}", poly);
    }

    let scene = scene3d.to_2d_scene();
    write_svg(&scene, "repro_new_missing_line.svg");
}