// base
// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
use na::Vec3;
use aabb::AABB;
use ray::Ray;
use utils::{ffmax, ffmin};
use std::sync::Arc;
use nalgebra::Dot;
use material::Scatter;
use objects::{HitRecord, Hitable};
use material;
struct Node<'a> {
    aabb_box: &'a Arc<Hitable>,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}
impl<'a> Hitable for Node<'a>{
    fn material(&self) -> Arc<Scatter>{
        //refactor
        Arc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))
    }
    fn hit(&self, ray: &Ray, t_min: &f32, t_max: &f32) ->Option<HitRecord>{
        match self.aabb_box.hit(ray, t_min, t_max) {
            Some(record) =>{
                let left_hit = if self.left.is_some(){
                    self.left.unwrap().hit(ray,t_min, t_max)
                } else{
                    None
                };
                let right_hit = if self.left.is_some(){
                    self.right.unwrap().hit(ray,t_min, t_max)
                } else{
                    None
                };
                match (left_hit, right_hit){
                    (left, None) => left,
                    (None, right) => right,
                    (left, right)=>{
                        if left.unwrap().t < right.unwrap().t{
                            left
                        }else{
                            right
                        }
                    }
                    _ => None
                }
            }
            None => None
        }
    }
    fn bounding_box(&self, t0: f32, t1: f32) -> AABB{
        self.aabb_box
    }
}
impl<'a> Node<'a> {
    pub fn new(mut self, value: &'a AABB) {
        let target_node = if *value < self.aabb_box{
            &mut self.left
        } else {
            &mut self.right
        };
        match target_node {
            &mut Some(ref mut sub) => sub.new(value),
            &mut None => {
                let node = Node {
                    aabb_box: value,
                    left: None,
                    right: None,
                };
                let boxed = Some(Box::new(node));
                *target_node = boxed;
            }
        }
    }
}
