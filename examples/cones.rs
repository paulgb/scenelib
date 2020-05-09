use scenelib::prelude::*;

fn draw_cone(scene: &mut Scene, place: Point2f, scale: f64) {
    let mut c = place;
    let mut size = Vec2f::new(160., 60.) * scale;
    let dir = Vec2f::new(0., -6.) * scale;

    for _ in 1..50 {
        scene.add_poly(&ellipse(c, size, 100));
        size = size * 0.94;
        c = c + dir;
    }
}

fn main() {
    let mut scene = scenelib::scene::Scene::new();
    let offset = 320.;
    let origin = Point2f::new(0., 0.);

    for j in (1..5).rev() {
        for i in 1..j {
            let x = (i as f64 - (j as f64) / 1.8) * offset;
            let y = j as f64 * -60.;
            let scale = 1.4 - (j as f64 * 0.17);
            draw_cone(&mut scene, origin + Vec2f::new(x, y), scale);
        }
    }

    scene.to_svg("cones.svg");
}
