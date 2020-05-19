use crate::geom::polygon::Polygon;
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

    pub fn append(&mut self, polys: &mut Vec<Polygon3>) {
        self.polys.append(polys)
    }

    pub fn project(self, projection: &dyn Transform) -> Vec<Polygon> {
        let mut v: Vec<(f64, Polygon)> = self.polys.into_iter().map(|d| {
            let d = d.apply(projection);

            let mut s = 0.;
            let mut c = 0;

            // TODO: this is a hack.
            for p in d.points.iter() {
                s += p.z;
                c += 1;
            }

            (s / c as f64, d.to_2d())
        }).collect();

        v.sort_by(|x, y| dangerous_compare(&x.0, &y.0));

        v.into_iter().map(|d|
            d.1
        ).collect()
    }

    pub fn to_2d_scene(self, projection: &dyn Transform) -> Scene {
        let mut s = Scene::new();
        for poly in self.project(projection) {
            s.add_poly(&poly)
        }

        s
    }
}