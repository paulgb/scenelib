use wasm_bindgen::prelude::*;

use crate::types::vector::Vector;
use na::{Point2, Point3};

pub trait PointContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

pub trait PointActions {
    fn translate(self, amount: Vector) -> Self;
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
