use crate::geom::types::{Point2f, Vec2f};
use na::{Rotation2};
pub trait Slope {
    fn slope(&self) -> f64;
}

impl Slope for Vec2f {
    fn slope(&self) -> f64 {
        self.y / self.x
    }
}

pub trait Rotate {
    fn rotate(&self, center: Point2f, radians: f64) -> Self;
}

impl Rotate for Point2f {
    fn rotate(&self, center: Point2f, radians: f64) -> Point2f {
        let p: Vec2f = (Rotation2::new(radians) * (self - center)).into();
        center + p
    }
}


pub trait XYFlip {
    fn xy_flip(&self) -> Self;
}

impl XYFlip for Point2f {
    fn xy_flip(&self) -> Point2f {
        self.yx()
    }
}

impl XYFlip for Vec2f {
    fn xy_flip(&self) -> Vec2f {
        self.yx()
    }
}

pub trait Translate {
    fn translate(&self, dist: Vec2f) -> Self;
}

impl Translate for Point2f {
    fn translate(&self, dist: Vec2f) -> Point2f {
        *self + dist
    }
}

pub trait FromAngle {
    fn from_angle(radians: f64) -> Self;
}

impl FromAngle for Vec2f {
    fn from_angle(radians: f64) -> Self {
        Vec2f::new(radians.cos(), radians.sin())
    }
}