extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use std::f32;
use utils::unit_vector;
use utils::random_in_unit_sphere;

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub vertical: Vec3<f32>,
    pub horizon: Vec3<f32>,
    pub lens_raidus: f32,
}

impl Camera {
    pub fn new_focus(look_from: Vec3<f32>,
                     look_at: Vec3<f32>,
                     vup: Vec3<f32>,
                     vfov: f32,
                     aspect: f32,
                     aperture: f32,
                     focus_dist: f32)
                     -> Self {
        let lens_raidus = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let looking = look_from - look_at;
        let w = unit_vector(&looking);
        let u = nalgebra::cross(&vup, &w);
        let v = nalgebra::cross(&w, &u);
        Camera {
            // look from is the same as origin
            lower_left_corner: look_from - u * focus_dist * half_width -
                               v * focus_dist * half_height -
                               focus_dist * w,
            origin: look_from,
            horizon: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
        }

    }
    pub fn new_positionable(look_from: Vec3<f32>,
                            look_at: Vec3<f32>,
                            vup: Vec3<f32>,
                            vfov: f32,
                            aspect: f32)
                            -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let looking = look_from - look_at;
        let w = unit_vector(&looking);
        let u = nalgebra::cross(&vup, &w);
        let v = nalgebra::cross(&w, &u);
        Camera {
            // look from is the same as origin
            lower_left_corner: look_from - u * half_width - v * half_height - w,
            origin: look_from,
            horizon: u * 2.0 * half_width,
            vertical: v * 2.0 * half_height,
            lens_raidus: 0.0,
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
