//! Three dimensional scenes.

use crate::draw_mode::DrawMode;
use crate::geom::polygon::Polygon;
use crate::projection::apply::Apply;
use crate::projection::form::Form;
use crate::projection::isometric::isometric_projection;
use crate::projection::polygon3::Polygon3;
use crate::projection::transform::Transform;
use crate::scene::Scene;
use na::Rotation3;

/// Represents a 3D scene as a set of 3D polygons (with associated
/// draw modes). Also acts as a builder for a 2D scene by storing
/// a rotation and perspective.
pub struct Scene3 {
    /// Polygons and drawing instructions.
    pub polys: Vec<(Polygon3, DrawMode)>,
    /// Perspective to apply when converting to 2D.
    pub perspective: f64,
    /// Rotation to apply when converting to 2D.
    pub projection: Rotation3<f64>,
}

fn dangerous_compare(x: &f64, y: &f64) -> std::cmp::Ordering {
    x.partial_cmp(y).unwrap()
}

impl Scene3 {
    /// Create a new 3D scene with an isometric projection (no perspective).
    pub fn new() -> Scene3 {
        Scene3 {
            polys: Vec::new(),
            perspective: 1.0,
            projection: isometric_projection(),
        }
    }

    /// Set the perspective based on distance from the camera to the origin.
    pub fn camera_distance(mut self, camera_distance: f64) -> Scene3 {
        self.perspective = 1. - (1. / camera_distance);
        self
    }

    /// Add a 3D polygon to the scene with the default draw mode.
    pub fn add_poly(&mut self, poly: Polygon3) {
        self.polys.push((poly, Default::default()));
    }

    /// Add a 3D polygon to the scene with the given pen.
    pub fn add_poly_with_pen(&mut self, poly: Polygon3, draw_mode: DrawMode) {
        self.polys.push((poly, draw_mode));
    }

    /// Add a 3D shape to the scene.
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
            .map(|d| (d.0.center.z, d.0.to_2d(self.perspective), d.1.clone()))
            .collect();

        v.sort_by(|x, y| dangerous_compare(&x.0, &y.0));

        v.into_iter().map(|d| (d.1, d.2)).collect()
    }

    /// Project the scene into 2D.
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
