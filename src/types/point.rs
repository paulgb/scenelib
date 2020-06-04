//! Aliases and adds utility methods to `Point2`.

use crate::types::to_string::ToString;
use crate::types::vector::Vector;
use na::{Point2, Point3};

/// Indicates that a struct contains points and can be manipulated
/// by passing a point modifier.
pub trait PointContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

pub trait PointActions {
    /// Translate contained points by a given vector.
    fn translate(self, amount: Vector) -> Self;
    /// Scale contained points with respect to the origin by a uniform amount.
    fn scale(self, amount: f64) -> Self;
    /// Scale contained points with respect to the origin by a given vector.
    fn scale2(self, amount: Vector) -> Self;
    /// Rotate contained points around the origin by an amount in radians.
    fn rotate(self, amount: f64) -> Self;
    /// Flip contained points along the x=y axis.
    fn xy_flip(self) -> Self;
}

pub type Point = Point2<f64>;

impl PointContainer for Point {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Point {
        lambda(self)
    }
}

impl<T> PointActions for T
where
    T: PointContainer,
{
    fn scale(self: T, amount: f64) -> Self {
        self.apply(&|x| x * amount)
    }

    fn scale2(self: T, amount: Vector) -> Self {
        self.apply(&|x| Point::new(x.x * amount.x, x.y * amount.y))
    }

    fn rotate(self: T, radians: f64) -> Self {
        let cosr = radians.cos();
        let sinr = radians.sin();
        self.apply(&|p| Point::new(p.x * cosr - p.y * sinr, p.x * sinr + p.y * cosr))
    }

    fn translate(self: T, amount: Vector) -> Self {
        self.apply(&|p| p + amount)
    }

    fn xy_flip(self: T) -> Self {
        self.apply(&|p| Point::new(p.y, p.x))
    }
}

pub type Point3f = Point3<f64>;

impl ToString for Point {
    fn to_string(&self) -> String {
        format!("pt({}, {})", self.x, self.y)
    }
}

/// Constructor for a 2D point.
pub fn pt(x: f64, y: f64) -> Point {
    Point::new(x, y)
}
