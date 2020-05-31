#[macro_use]
extern crate scenelib;
use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();

    scene.add_segment(LineSegment::new(
        pt(0.01934741332132946, 0.4503200744352772),
        pt(-0.21972214752293184, 0.611218361274748),
    ));

    let polys = vec![Polygon::from_coords(vec![
        (-0.000000000000000040274189285034336, 0.2776784803719598),
        (-0.00000000000000003900535185736784, 0.4930387478367542),
        (-0.20039718173864648, 0.6247765808369093),
        (-0.20713525621191442, 0.406604626170877),
    ])];

    for poly in polys {
        scene.add_poly(&poly);
    }

    scene
        .to_plot()
        .write_svg(&svg_filename!());
}
