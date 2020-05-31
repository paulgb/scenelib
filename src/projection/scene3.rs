use crate::geom::polygon::Polygon;
use crate::projection::apply::Apply;
use crate::projection::form::Form;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;
use crate::scene::Scene;

pub struct Scene3 {
    pub polys: Vec<Polygon3>,
}

fn dangerous_compare(x: &f64, y: &f64) -> std::cmp::Ordering {
    x.partial_cmp(y).unwrap()
}

impl Scene3 {
    pub fn new() -> Scene3 {
        Scene3 { polys: Vec::new() }
    }

    pub fn add_poly(&mut self, poly: Polygon3) {
        self.polys.push(poly)
    }

    pub fn add_form(&mut self, mut form: Form) {
        self.polys.append(&mut form.polys)
    }

    pub fn project(&self, perspective: f64) -> Vec<Polygon> {
        let mut v: Vec<(f64, Polygon)> = self
            .polys
            .iter()
            .map(|d| {
                let mut s = 0.;
                let mut c = 0;

                // TODO: this is a hack.
                for p in d.points.iter() {
                    s += p.z;
                    c += 1;
                }

                (s / c as f64, d.to_2d(perspective))
            })
            .collect();

        v.sort_by(|x, y| dangerous_compare(&x.0, &y.0));

        v.into_iter().map(|d| d.1).collect()
    }

    pub fn to_2d_scene(&self) -> Scene {
        self.to_2d_scene_with_perspective(0.0)
    }

    pub fn to_2d_scene_with_perspective(&self, perspective: f64) -> Scene {
        let mut s = Scene::new();
        for poly in self.project(perspective) {
            s.add_poly(&poly)
        }

        s
    }
}

impl Apply for Scene3 {
    fn apply(self, transform: &dyn Transform) -> Scene3 {
        let polys = self.polys.into_iter().map(|d| d.apply(transform)).collect();
        Scene3 { polys }
    }
}
