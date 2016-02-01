extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use std::rc::Rc;
use nalgebra::Dot;
use material::Scatter;

pub struct HitableList {
    // should use trait and generics
    pub list: Vec<Sphere>,
}
impl HitableList {
    // I think can do a from/to vec to hitable maybe. for now give it nothing and push
    pub fn new() -> Self {
        let list = Vec::new();
        HitableList { list: list }
    }
    pub fn len(&self) -> usize {
        self.list.len()
    }
    pub fn push(&mut self, sphere: Sphere) {
        self.list.push(sphere);
    }
    pub fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<(HitRecord, Rc<Scatter>)> {
        let mut closest_so_far = t_max.clone();
        let mut last_hit: Option<(HitRecord, Rc<Scatter>)> = None;
        for object in &self.list {
            if let Some(record) = object.hit(ray, t_min, &closest_so_far) {
                closest_so_far = record.t;
                last_hit = Some((record, object.material.clone()));
            }
        }
        last_hit
    }
}
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3<f32>,
    pub normal: Vec3<f32>,
}
impl HitRecord {
    pub fn new(t: f32, p: Vec3<f32>, normal: Vec3<f32>) -> Self {
        HitRecord {
            p: p,
            t: t,
            normal: normal,
        }
    }
}

// not really the same. I should make a trait called hitable
pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
    pub material: Rc<Scatter>,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32, material: Rc<Scatter>) -> Self {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord> {
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
