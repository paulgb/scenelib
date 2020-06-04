use crate::draw_mode::DrawMode;
use crate::geom::polygon::Polygon;
use crate::projection::apply::Apply;
use crate::projection::form::Form;
use crate::projection::isometric::isometric_projection;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;
use crate::scene::Scene;
use na::Rotation3;

pub struct Scene3 {
    pub polys: Vec<(Polygon3, DrawMode)>,
    pub perspective: f64,
    pub projection: Rotation3<f64>,
}

fn dangerous_compare(x: &f64, y: &f64) -> std::cmp::Ordering {
    x.partial_cmp(y).unwrap()
}

impl Scene3 {
    pub fn new() -> Scene3 {
        Scene3 {
            polys: Vec::new(),
            perspective: 1.0,
            projection: isometric_projection(),
        }
    }

    pub fn camera_distance(mut self, camera_distance: f64) -> Scene3 {
        self.perspective = 1. - (1. / camera_distance);
        self
    }

    pub fn add_poly(&mut self, poly: Polygon3) {
        self.polys.push((poly, Default::default()));
    }

    pub fn add_form(&mut self, form: Form) {
        let draw_mode = form.draw_mode.clone();
        self.polys.append(
            &mut form
                .polys
                .into_iter()
                .map(|d| (d, draw_mode.clone()))
                .collect(),
        )
    }

    fn project(&self) -> Vec<(Polygon, DrawMode)> {
        let mut v: Vec<(f64, Polygon, DrawMode)> = self
            .polys
            .iter()
            .map(|d| {
                let mut s = 0.;
                let mut c = 0;

                // TODO: this is a hack.
                for p in d.0.points.iter() {
                    s += p.z;
                    c += 1;
                }

                (s / c as f64, d.0.to_2d(self.perspective), d.1.clone())
            })
            .collect();

        v.sort_by(|x, y| dangerous_compare(&x.0, &y.0));

        v.into_iter().map(|d| (d.1, d.2)).collect()
    }

    pub fn to_2d(self) -> Scene {
        let mut s = Scene::new();

        // TODO: this is hacky
        let proj = self.projection.clone();

        for (poly, draw_mode) in self.apply(&proj).project() {
            s.add_poly_with_draw_mode(&poly, draw_mode)
        }

        s
    }
}

impl Apply for Scene3 {
    fn apply(mut self, transform: &dyn Transform) -> Scene3 {
        self.polys = self
            .polys
            .into_iter()
            .map(|(poly, draw_mode)| (poly.apply(transform), draw_mode))
            .collect();
        self
    }
}
