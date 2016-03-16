extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use std::sync::Arc;
use nalgebra::Dot;
use material::Scatter;
use objects::{HitRecord, Hitable};
pub struct MovingSphere {
    pub center0: Vec3<f32>,
    pub center1: Vec3<f32>,
    pub radius: f32,
    material: Arc<Scatter>,
    pub time0: f32,
    pub time1: f32,
}

impl MovingSphere {
    pub fn new(center0: Vec3<f32>,
               center1: Vec3<f32>,
               radius: f32,
               material: Arc<Scatter>,
               time0: f32,
               time1: f32)
               -> Self {
        MovingSphere {
            center0: center0,
            center1: center1,
            radius: radius,
            material: material,
            time0: time0,
            time1: time1,
        }
    }
    pub fn center(&self, time: f32) -> Vec3<f32> {
        self.center0 +
        (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hitable for MovingSphere {
    fn material(&self) -> Arc<Scatter> {
        self.material.clone()
    }
    fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord> {
        let origin = ray.origin;
        let direction = ray.direction;
        let mut return_value: Option<HitRecord> = None;
        let center_at_time = self.center(ray.time);

        let oc = origin - center_at_time;
        let a = direction.dot(&direction);
        let b = oc.dot(&direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminate = b * b - a * c;
        if discriminate > 0.0 {
            let sqrt_discriminate = discriminate.sqrt();
            let temp = ((0.0 - b) - sqrt_discriminate) / a;
            if temp < *t_max && temp > *t_min {
                let point = ray.point_at_parameter(temp);
                let normal = (point - center_at_time) / self.radius;
                let hit = HitRecord::new(temp, point, normal);
                return_value = Some(hit)
            } else {
                let temp = ((0.0 - b) + sqrt_discriminate) / a;
                if temp < *t_max && temp > *t_min {
                    let point = ray.point_at_parameter(temp);
                    let normal = (point - center_at_time) / self.radius;
                    let hit = HitRecord::new(temp, point, normal);
                    return_value = Some(hit)
                }
            }
        };
        return_value
    }
}

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    material: Arc<Scatter>,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, material: Arc<Scatter>) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}
impl Hitable for Sphere {
    fn material(&self) -> Arc<Scatter> {
        self.material.clone()
    }
    fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord> {
        let origin = ray.origin;
        let direction = ray.direction;
        let oc = origin - self.center;
        let a = direction.dot(&direction);
        let b = 2.0 * oc.dot(&direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminate = b * b - 4.0 * a * c;
        // clean up the crazy tree.
        if discriminate < 0.0 {
            None
        } else {
            let ds = discriminate.sqrt();
            let temp_neg = (0.0 - b - ds) / (2.0 * a);
            // calculating pos most of the time is a waste?
            let temp_pos = (0.0 - b + ds) / (2.0 * a);
            // is it wasteful to use an option to dedup code?
            let temp = if temp_neg < *t_max && temp_neg > *t_min {
                Some(temp_neg)
            } else if temp_pos < *t_max && temp_pos > *t_min {
                Some(temp_pos)
            } else {
                None
            };

            match temp {
                Some(temp) => {
                    let point = ray.point_at_parameter(temp);
                    let normal = (point - self.center) / self.radius;
                    let hit = HitRecord::new(temp, point, normal);
                    Some(hit)
                }
                None => None,
            }
        }
    }
}
