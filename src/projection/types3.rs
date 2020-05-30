pub type Vector3 = na::Vector3<f64>;
pub type Point3 = na::Point3<f64>;

pub fn vec3(x: f64, y: f64, z: f64) -> Vector3 {
    Vector3::new(x, y, z)
}

pub fn pt3(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}