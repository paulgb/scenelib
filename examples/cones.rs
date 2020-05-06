use scenelib::prelude::*;

fn draw_cone(scene: &mut Scene, place: Coord, scale: f64) {
    let mut c = place;
    let mut size = Vector::new(160., 60.) * scale;
    let dir = Vector::new(0., -6.);

    for _ in 1..50 {
        scene.add_poly(&ellipse(c, size, 100));
        size = size * 0.94;
        c = c + dir;
    }
}

fn main() {
    let mut scene = scenelib::scene::Scene::new();
    let offset = 320.;

    for j in (1..5).rev() {
        for i in 1..j {
            let x = (i as f64 - (j as f64) / 2.) * offset;
            let y = j as f64 * -80.;
            let scale = 1. + (j as f64 * 0.05);
            draw_cone(&mut scene, ORIGIN + Vector::new(x, y), scale);
        }
    
    }


    scene.to_svg("cones.svg");
}
