//! Suite of useful imports for sketching.

pub use std::f64::consts::PI;
pub const TWO_PI: f64 = PI * 2.;

pub use crate::draw_mode::{fill_only, pen, stroke};
pub use crate::export::WriteSVG;
pub use crate::geom::line_segment::LineSegment;
pub use crate::geom::polygon::Polygon;
pub use crate::noise::NoiseMaker;
pub use crate::plot::Plot;
pub use crate::projection::apply::{Apply, ApplyOps};
pub use crate::projection::form::Form;
pub use crate::projection::platonic::{cube, tetrahedron};
pub use crate::projection::polygon3::Polygon3;
pub use crate::projection::scene3::Scene3;
pub use crate::projection::types3::{pt3, vec3};
pub use crate::scene::Scene;
pub use crate::shape::{circle, square};
pub use crate::types::{pt, vec, Point, PointActions, Vector, VectorExtension};
