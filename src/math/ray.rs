use super::vec3::{Point3, Vec3};

#[allow(dead_code)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at<T: Into<f64>>(&self, t: T) -> Point3 {
        self.origin + (self.direction * t)
    }
}
