extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use std::f32;
use utils::{unit_vector, random_unit_disk};

pub struct Camera {
    pub origin: Vec3<f32>,
    pub lower_left_corner: Vec3<f32>,
    pub vertical: Vec3<f32>,
    pub horizon: Vec3<f32>,
    pub lens_raidus: f32,
    pub u: Vec3<f32>,
    pub v: Vec3<f32>,
    pub w: Vec3<f32>,
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
            lower_left_corner: look_from - u * focus_dist * half_width -
                               v * focus_dist * half_height -
                               w * focus_dist,
            origin: look_from,
            horizon: u * 2.0 * half_width * focus_dist,
            vertical: v * 2.0 * half_height * focus_dist,
            lens_raidus: lens_raidus,
            u: u,
            v: v,
            w: w,
        }

    }

    pub fn get_ray(&self, s: &f32, t: &f32) -> Ray {
        let rd = random_unit_disk() * self.lens_raidus;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
                 self.lower_left_corner + self.horizon * *s + self.vertical * *t - self.origin -
                 offset)
    }
}
