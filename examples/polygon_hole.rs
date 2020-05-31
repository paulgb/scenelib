#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();

    for i in 0..40 {
        scene.add_segment(LineSegment::new(
            pt(-5., -5. + i as f64),
            pt(20., -10. + i as f64),
        ));
    }

    let poly = Polygon::with_holes(
        vec![pt(0., 0.), pt(0., 15.), pt(15., 15.), pt(15., 0.)],
        vec![vec![pt(10., 5.), pt(10., 10.), pt(5., 10.), pt(5., 5.)]],
    );
    scene.add_poly(&poly);

    scene.to_plot().write_svg(&svg_filename!());
}
