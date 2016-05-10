extern crate rand;
use rand::distributions::{IndependentSample, Range};
use nalgebra;
use nalgebra::Vec3;
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
    pub time0: f32,
    pub time1: f32,
}

pub fn book_cam(image_x: &u32, image_y: &u32, offset: f32) -> Camera {
    let look_from = Vec3::new(13.0, 3.0, -3.0);
    let look_at = Vec3::new(0.0, -4.0, 0.0);
    let distance = 10.0;
    let aperture = 0.0;
    Camera::new_focus(look_from,
                      look_at,
                      Vec3::new(0.0, 1.0, 0.0),
                      20.0,
                      *image_x as f32 / *image_y as f32,
                      aperture,
                      distance,
                      0.0,
                      1.0)
}
pub fn normal_cam2(image_x: &u32,
               image_y: &u32,
               offset_x: f32,
               offset_y: f32,
               offset_z: f32)
               -> Camera {
    let look_from = Vec3::new(3.0 + offset_x, 3.0 + offset_y, 2.0 + offset_z);
    let look_at = Vec3::new(0.0, 0.0, 0.0 - 1.0);
    let distance = (look_from - look_at).len() as f32;
    let aperture = 0.0;
    Camera::new_focus(look_from,
                      look_at,
                      Vec3::new(0.0, 1.0, 0.0),
                      20.0,
                      *image_x as f32 / *image_y as f32,
                      aperture,
                      distance,
                      0.0,
                      0.0)
}

pub fn head_on_cam(image_x: &u32,
               image_y: &u32,
               offset_x: f32,
               offset_y: f32,
               offset_z: f32)
               -> Camera {
    Camera::new_set()
}

pub fn normal_cam(image_x: &u32, image_y: &u32, offset_x: f32, offset_y: f32, offset_z: f32) -> Camera {
    let look_from = Vec3::new(13.0, 2.0 + offset_y, 3.0 + offset_z);
    let look_at = Vec3::new(0.0 + offset_x, 0.0, 0.0);

    let distance = 10.0;
    let aperture = 0.0;
    Camera::new_focus(look_from,
                      look_at,
                      Vec3::new(0.0, 1.0, 0.0),
                      20.0,
                      *image_x as f32 / *image_y as f32,
                      aperture,
                      distance,
                      0.0,
                      1.0)

}
impl Camera {
    pub fn new_set() -> Self {

        let vup = Vec3::new(0.0, 0.0, 0.0);
        let vfov = 1.0;
        let aspect = 45.0;
        let aperture = 0.0;
        let focus_dist = 0.0;
        let time0 = 0.0;
        let time1 = 0.0;
        let look_from = Vec3::new(1.0, 0.0, 0.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);

        let lens_raidus = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let looking = look_from - look_at;
        let w = unit_vector(&looking);
        let u = nalgebra::cross(&vup, &w);
        let v = nalgebra::cross(&w, &u);
        Camera {
            lower_left_corner: Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
            horizon: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            lens_raidus: lens_raidus,
            u: u,
            v: v,
            w: w,
            time0: time0,
            time1: time1,
        }

    }
    pub fn new_focus(look_from: Vec3<f32>,
                     look_at: Vec3<f32>,
                     vup: Vec3<f32>,
                     vfov: f32,
                     aspect: f32,
                     aperture: f32,
                     focus_dist: f32,
                     time0: f32,
                     time1: f32)
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
            time0: time0,
            time1: time1,
        }

    }

    pub fn get_ray(&self, s: &f32, t: &f32) -> Ray {
        let mut rng = rand::thread_rng();
        let random_index = Range::new(0.0, 1.0);

        let rands = random_index.ind_sample(&mut rng);
        let rd = random_unit_disk() * self.lens_raidus;
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time0 + rands * (self.time1 - self.time0);
        Ray::new(self.origin + offset,
                 self.lower_left_corner + self.horizon * *s + self.vertical * *t - self.origin -
                 offset,
                 time)
    }
}
