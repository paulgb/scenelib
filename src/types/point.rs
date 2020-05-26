use wasm_bindgen::prelude::*;

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
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    #[wasm_bindgen(skip)]
    pub inner: Point2<f64>,
}

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
        self.apply(&|x| Point {
            inner: x.inner * amount,
        })
    }

    fn rotate(self: T, radians: f64) -> Self {
        let cosr = radians.cos();
        let sinr = radians.sin();
        self.apply(&|x| Point {
            inner: [x.inner.x * cosr - x.inner.y * sinr, x.inner.x * sinr + x.inner.y * cosr].into(),
        })
    }

    fn translate(self: T, amount: Vector) -> Self {
        self.apply(&|x| Point {
            inner: x.inner + amount.inner,
        })
    }

    fn xy_flip(self: T) -> Self {
        self.apply(&|x| Point {
            inner: [x.inner.y, x.inner.x].into(),
        })
    }
}

pub type Point3f = Point3<f64>;

impl From<Vector> for Point {
    fn from(v: Vector) -> Point {
        Point {
            inner: v.inner.into(),
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pt({}, {})", self.inner.x, self.inner.y)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            inner: self.inner + rhs.inner
        }
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            inner: self.inner / rhs
        }
    }
}


#[wasm_bindgen]
pub fn pt(x: f64, y: f64) -> Point {
    Point { inner: [x, y].into() }
}