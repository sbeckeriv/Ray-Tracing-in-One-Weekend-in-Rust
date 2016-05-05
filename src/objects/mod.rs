extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use std::sync::Arc;
use material::Scatter;
pub mod sphere;
pub mod bvh;
pub trait Hitable: Send + Sync{
    fn material(&self) -> Arc<Scatter>;
    fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> (Vec3<f32>, Vec3<f32>);
    fn overlaps_bounding_box(&self, min: Vec3<f32>, max: Vec3<f32>) -> bool {
        let local = self.bounding_box(0.0, 0.0);
        let results = local.0.x <= max.x && min.x <= local.1.x ||
                      local.0.y <= max.y && min.y <= local.1.y ||
                      local.0.z <= max.z && min.z <= local.1.z;
        //      let results = (local.0.x <= min.x && local.1.x >= min.x ||
        //                     local.0.x <= max.x && local.1.x >= min.x) &&
        //                    (local.0.y <= min.y && local.1.y >= min.y ||
        //                     local.0.y <= max.y && local.1.y >= min.y) &&
        //                    (local.0.z <= min.z && local.1.z >= min.z ||
        //                     local.0.z <= max.z && local.1.z >= min.z);
        // println!("{:?} :: {:?} :: {:?}", results, local, (min, max));
        results
    }
}
pub struct HitableList {
    pub list: Vec<Arc<Hitable>>,
}
impl HitableList {
    // I think can do a from/to vec to hitable maybe. for now give it nothing and push
    pub fn new() -> Self {
        let list = Vec::new();
        HitableList { list: list }
    }
    pub fn push(&mut self, sphere: Arc<Hitable>) {
        self.list.push(sphere);
    }
    pub fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) -> Option<(HitRecord, Arc<Scatter>)> {
        let mut closest_so_far = t_max.clone();
        let mut last_hit: Option<(HitRecord, Arc<Scatter>)> = None;
        for object in &self.list {
            if let Some(record) = object.hit(ray, t_min, &closest_so_far) {
                closest_so_far = record.t;
                last_hit = Some((record, object.material()));
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
