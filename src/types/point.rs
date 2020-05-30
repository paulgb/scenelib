use crate::types::to_string::ToString;
use crate::types::vector::Vector;
use na::{Point2, Point3};

pub trait PointContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

pub trait PointActions {
    fn translate(self, amount: Vector) -> Self;
    fn scale(self, amount: f64) -> Self;
    fn rotate(self, amount: f64) -> Self;
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

pub fn pt(x: f64, y: f64) -> Point {
    Point::new(x, y)
}
