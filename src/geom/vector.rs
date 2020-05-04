#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Vector {
        Vector {
            x, y
        }
    }

    pub fn slope(&self) -> f64 {
        self.y / self.x
    }

    pub fn rotate(&self, radians: f64) -> Vector {
        Vector::new(
            self.x * radians.sin() + self.y * radians.cos(),
            self.x * radians.cos() + -self.y * radians.sin()
        )
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector::new(self.x / other, self.y / other)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector::new(self.x * other, self.y * other)
    }
}