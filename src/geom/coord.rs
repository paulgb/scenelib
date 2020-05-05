use crate::geom::vector::Vector;
use crate::geom::traits::{Rotate, Translate, XYFlip};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Coord {
    pub x: f64,
    pub y: f64
}

impl std::cmp::Eq for Coord {
}

impl std::cmp::Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.x, self.y).partial_cmp(&(other.x, other.y)).unwrap()
    }
}

impl Coord {
    pub fn new(x: f64, y: f64) -> Coord {
        Coord {x, y}
    }
}

impl std::ops::Sub for Coord {
    type Output = Vector;

    fn sub(self, other: Coord) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Sub<Vector> for Coord {
    type Output = Coord;

    fn sub(self, other: Vector) -> Coord {
        Coord::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Add<Vector> for Coord {
    type Output = Coord;

    fn add(self, other: Vector) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}

impl Rotate<Coord> for Coord {
    fn rotate(&self, center: Coord, radians: f64) -> Coord {
        let mut v: Vector = *self - center;
        v = v.rotate(radians);
        center + v
    }
}

impl Translate<Coord> for Coord {
    fn translate(&self, dist: Vector) -> Coord {
        *self + dist
    }
}

impl XYFlip<Coord> for Coord {
    fn xy_flip(&self) -> Coord {
        Coord::new(self.y, self.x)
    }
}