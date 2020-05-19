use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();
    let origin = Point2f::new(20., 40.);
    let size = Vec2f::new(40., 20.);
    let center = origin + size / 2.;

    for i in 0..40 {
        let s1 = rect(origin, size);
        scene.add_segment(
            LineSegment::new(
                Point2f::new(-5., -5. + i as f64),
                Point2f::new(20., -10. + i as f64)
            )
        );
    }

    let poly = Polygon::with_holes(
        vec![
            Point2f::new(0., 0.),
            Point2f::new(0., 15.),
            Point2f::new(15., 15.),
            Point2f::new(15., 0.),
        ],
        vec![vec![
            Point2f::new(10., 5.),
            Point2f::new(10., 10.),
            Point2f::new(5., 10.),
            Point2f::new(5., 5.),
            ]],
    );
    scene.add_poly(&poly);
    
    scene.to_svg("polygon_hole.svg");
}
