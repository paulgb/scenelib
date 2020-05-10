pub use std::f64::consts::PI;
pub const TWO_PI: f64 = PI * 2.;

pub use crate::geom::types::{Point2f, Point3f, Vec2f, Vec3f};
pub use crate::geom::line_segment::LineSegment;
pub use crate::geom::polygon::Polygon;
pub use crate::geom::traits::{Rotate, Translate, FromAngle};
pub use crate::scene::Scene;
pub use crate::shape::{circle, rect, ellipse};
pub use crate::projection::polygon3::{Polygon3, Projection};
pub use crate::projection::form::cube;