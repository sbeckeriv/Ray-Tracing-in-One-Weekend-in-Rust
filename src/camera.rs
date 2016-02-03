extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use std::f32;

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub vertical: Vec3<f32>,
    pub horizon: Vec3<f32>,
}

impl Camera {
    pub fn new_positionable(vfov: f32, aspect: f32) -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(half_width * (0.0 - 1.0),
                                         half_height * (0.0 - 1.0),
                                         0.0 - 1.0),

            vertical: Vec3::new(0.0, 2.0 * half_width, 0.0),
            horizon: Vec3::new(2.0 * half_height, 0.0, 0.0),
        }

    }
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
    pub fn get_ray(&self, u: &f32, v: &f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + self.horizon * *u + self.vertical * *v - self.origin)
    }
}
