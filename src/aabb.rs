use utils::{ffmin, ffmax};
use ray::Ray;

extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
pub struct AABB {
    pub min: Vec3<f32>,
    pub max: Vec3<f32>,
}
impl AABB {
    pub fn new(min: Vec3<f32>, max: Vec3<f32>) -> Self {
        AABB {
            min: min,
            max: max,
        }
    }

    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in (0..3) {
            let t0 = ffmin((self.min[a] - ray.origin[a]) / ray.direction[a],
                           (self.max[a] - ray.origin[a]) / ray.direction[a]);

            let t1 = ffmax((self.min[a] - ray.origin[a]) / ray.direction[a],
                           (self.max[a] - ray.origin[a]) / ray.direction[a]);
            let min_min = ffmax(t0, tmin);
            let max_max = ffmin(t1, tmax);
            if max_max <= min_min {
                return false;
            }
        }
        true
    }
}
