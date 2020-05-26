use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();

    scene.add_segment(LineSegment::new(
        Point2f::new(0.0000000000000000023739852701224147, -0.015003924708129003),
        Point2f::new(0.02371600484171645, -0.0013692441779467447),
    ));

    let polys = vec![Polygon::from_coords(vec![
        (0.02371600484171645, -0.0013692441779467447),
        (0.02370687830359989, 0.0013687172570229103),
        (-0.000000000000000002391410184998205, 0.015114052649583515),
        (-0.0000000000000000023923343742740626, 0.012370822083255595),
    ])];

    for poly in polys {
        scene.add_poly(&poly);
    }

    scene
        .to_plot()
        .write_svg("repro_new_missing_line_from_polys.svg");
}
