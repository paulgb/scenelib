use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();
    let origin = Point2f::new(20., 40.);
    let size = Vec2f::new(40., 20.);
    let center = origin + size / 2.;

    let s1 = rect(origin, size);
    scene.add_poly(&circle(origin + size * 3., 100., 100));

    for i in 0..40 {
        let s2 = s1
            .rotate(center + size / 2., i as f64 * (PI / 30.0))
            .translate(size * i as f64 / 10.);
        scene.add_poly(&s2);
    }

    scene.to_plot().write_svg("basic_shapes.svg");
}
