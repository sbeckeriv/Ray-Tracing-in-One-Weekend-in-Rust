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
    pub fn hit(min: Vec3<f32>, max: Vec3<f32>, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in (0..3) {
            let t0 = ffmin((min[a] - ray.origin[a]) / ray.direction[a],
                           (max[a] - ray.origin[a]) / ray.direction[a]);

            let t1 = ffmax((min[a] - ray.origin[a]) / ray.direction[a],
                           (max[a] - ray.origin[a]) / ray.direction[a]);
            let min_min = ffmax(t0, tmin);
            let max_max = ffmin(t1, tmax);
            if ray.debug {
                println!("aabb::{} {:?}", a, (min_min, max_max));
            }
            if max_max <= min_min {
                return false;
            }
        }
        true
    }
}
