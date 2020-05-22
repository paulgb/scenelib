use scenelib::prelude::*;

fn main() {
    let mut scene = Scene::new();
    let origin = Point2f::new(0., 0.);

    let mut pos = origin;

    for i in 0..100 {
        let y = i as f64 * 3. - 200.;
        scene.add_segment(LineSegment::new(
            Point2f::new(-300., y),
            Point2f::new(300., y + 20.),
        ))
    }

    let mut sp1: Vec<Point2f> = Vec::new();
    let mut sp2: Vec<Point2f> = Vec::new();

    for i in 10..400 {
        let angle = (i as f64 * (TWO_PI / 5.)).sqrt();
        pos = pos + Vec2f::from_angle(angle) * 10.;
        let p1 = pos + Vec2f::from_angle(angle + PI / 4.) * 20.;
        let p2 = pos + Vec2f::from_angle(angle - PI / 4.) * 20.;

        sp1.push(p1);
        sp2.push(p2);
    }

    sp2.reverse();
    sp1.append(&mut sp2);
    scene.add_poly(&Polygon::new(sp1));

    write_svg(&scene, "spiral.svg");
}
