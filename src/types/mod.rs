//! Aliases and extensions for 2D points and vectors.

pub mod point;
pub mod to_string;
pub mod vector;

pub use point::{pt, Point, PointActions, PointContainer};
pub use vector::{vec, Vector, VectorExtension};
