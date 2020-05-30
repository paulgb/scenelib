use scenelib::prelude::*;

fn main() {
    let mut scene3d: Scene3 = Scene3::new();

    const LAT_MIN: f64 = PI / 4.;
    const LAT_MAX: f64 = 3. * PI / 4.;
    const LAT_STEPS: u32 = 10;
    const LON_STEPS: u32 = 30;
    const RADIUS: f64 = 300.;
    const SCALE: f64 = 10.;

    for i in 0..LON_STEPS {
        for j in 0..LAT_STEPS {
            let lat = LAT_MIN + (j as f64 / LAT_STEPS as f64) * (LAT_MAX - LAT_MIN);
            let lon = (i as f64 / LON_STEPS as f64) * PI * 2.;

            let mut tet = tetrahedron()
                // Scale.
                .scale(SCALE)
                // Translate.
                .translate(vec3(RADIUS, 0., 0.))
                // Rotate.
                .rotate_euler(0., lon, lat);

            scene3d.append(&mut tet.polys);
        }
    }

    let scene = scene3d.apply(&isometric_projection()).to_2d_scene();
    let mut plot = scene.to_plot();
    println!("Before: {:?}", plot.cost());
    plot = greedy_optimize(plot);
    println!("After: {:?}", plot.cost());
    plot.write_svg("tetrasphere.svg");
}
