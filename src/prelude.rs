pub use std::f64::consts::PI;
pub const TWO_PI: f64 = PI * 2.;

pub use crate::geom::types::{Point2f, Vec2f};
pub use crate::geom::line_segment::LineSegment;
pub use crate::geom::polygon::Polygon;
pub use crate::geom::traits::{Rotate, Translate, FromAngle};
pub use crate::scene::Scene;
pub use crate::shape::{circle, rect, ellipse};
