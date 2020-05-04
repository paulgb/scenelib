use crate::geom::coord::Coord;
use crate::geom::vector::Vector;
pub trait Rotate<T> {
    fn rotate(&self, center: Coord, radians: f64) -> T;
}

pub trait Translate<T> {
    fn translate(&self, dist: Vector) -> T;
}

pub trait XYFlip<T> {
    fn xy_flip(&self) -> T;
}