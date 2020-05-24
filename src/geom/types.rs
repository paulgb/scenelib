use wasm_bindgen::prelude::*;

use na::{Point2, Point3, Vector2, Vector3};

pub trait PointContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

pub trait VectorContainer {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Self;
}

trait VectorActions {
}

impl<T> VectorActions for T where T: VectorContainer {

}

pub trait PointActions {
    fn translate(self, amount: Vector) -> Self;
    fn xy_flip(self) -> Self;
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    #[wasm_bindgen(skip)]
    pub inner: Point2<f64>
}

impl PointContainer for Point {
    fn apply(self, lambda: &dyn Fn(Point) -> Point) -> Point {
        lambda(self)
    }
}

impl<T> PointActions for T where T: PointContainer {
    fn translate(self: T, amount: Vector) -> Self {
        self.apply(&|x| Point { inner: x.inner + amount.inner })
    }

    fn xy_flip(self: T) -> Self {
        self.apply(&|x| Point { inner: [x.inner.y, x.inner.x].into() })
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    #[wasm_bindgen(skip)]
    pub inner: Vector2<f64>
}

impl Vector {
    pub fn slope(&self) -> f64 {
        (self.inner.y) / (self.inner.x)
    }

    pub fn from_angle(radians: f64) -> Vector {
        Vector {
            inner: Vector2::<f64>::new(
                radians.cos(),
                radians.sin()
            )
        }
    }
}

//pub type Vec2f = Vector2<f64>;
pub type Vec3f = Vector3<f64>;
//pub type Point2f = Point2<f64>;
pub type Point3f = Point3<f64>;

impl From<Vector> for Point {
    fn from(v: Vector) -> Point {
        Point {inner: v.inner.into()}
    }
}
