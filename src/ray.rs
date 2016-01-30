extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
pub struct Ray {
    pub origin: Vec3<f32>,
    pub direction: Vec3<f32>,
}
impl Ray {
    pub fn new(origin: Vec3<f32>, direction: Vec3<f32>) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3<f32> {
        // t should be whole numbers -1 0 1 2..
        self.origin + (self.direction * t)
    }
}
