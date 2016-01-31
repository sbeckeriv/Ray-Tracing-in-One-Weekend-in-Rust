extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub vertical: Vec3<f32>,
    pub horizon: Vec3<f32>,
}

impl Camera {
    pub fn new(origin: Vec3<f32>,
               lower_left_corner: Vec3<f32>,
               vertical: Vec3<f32>,
               horizon: Vec3<f32>)
               -> Self {
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            vertical: vertical,
            horizon: horizon,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + self.horizon * u + self.vertical * v - self.origin)
    }
}
