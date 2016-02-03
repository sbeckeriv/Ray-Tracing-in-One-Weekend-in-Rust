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
    pub fn new_positionable(look_from: Vec3<f32>,
                            look_at: Vec3<f32>,
                            vup: Vec3<f32>,
                            vfov: f32,
                            aspect: f32)
                            -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = look_from - look_at;
        let u = nalgebra::cross(&vup, &w);
        let v = nalgebra::cross(&w, &u);
        Camera {
            lower_left_corner: look_from - u * half_width - v * half_height - w,
            origin: look_from,
            vertical: u * 2.0 * half_height,
            horizon: v * 2.0 * half_height,
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
