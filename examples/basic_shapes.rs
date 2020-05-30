use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();

    let origin = vec(-40., -40.);
    let direction = vec(80., 80.);

    let s1 = square();
    scene.add_poly(&circle(40).scale(40.));

    for i in 0..40 {
        let s2 = s1
            .clone()
            .rotate(i as f64 * (PI / 30.0))
            .scale(30.)
            .translate(origin + direction * i as f64 / 10.);
        scene.add_poly(&s2);
    }

    write_svg(&scene.to_plot(), "basic_shapes.svg");
}
