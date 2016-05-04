// base
// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
use na::Vec3;
use ray::Ray;
use utils::{ffmax, ffmin};
use std::sync::Arc;
use nalgebra::Dot;
use material::Scatter;
use objects::{HitRecord, Hitable, HitableList};
use material;
enum HitDirection {
    Left,
    Right,
    Miss,
    End,
}
struct Node {
    pub min: Vec3<f32>,
    pub max: Vec3<f32>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub hitlist: HitableList,
}

impl Node {
    fn bounding_box(&self, t0: f32, t1: f32) -> (Vec3<f32>, Vec3<f32>) {
        let one = self.min;
        let two = self.max;
        (one, two)
    }
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitDirection {
        let mut hit = true;
        //Moved from aabb
        for a in (0..3) {
            let t0 = ffmin((self.min[a] - ray.origin[a]) / ray.direction[a],
                           (self.max[a] - ray.origin[a]) / ray.direction[a]);

            let t1 = ffmax((self.min[a] - ray.origin[a]) / ray.direction[a],
                           (self.max[a] - ray.origin[a]) / ray.direction[a]);
            let min_min = ffmax(t0, t_min);
            let max_max = ffmin(t1, t_max);
            if max_max <= min_min {
                hit = false;
            }
        }
        if hit {
            match (
                self.left.map_or(HitDirection::Miss, |n| n.hits(ray, t_min, t_max)),
                self.right.map_or(HitDirection::Miss, |n| n.hits(ray, t_min, t_max))

                   ) {
                (HitDirection::Miss, HitDirection::Miss) => HitDirection::End,
                (left, HitDirection::Miss) => HitDirection::Left,
                (HitDirection::Miss, right) => HitDirection::Right,
                _ => {
                    if true {
                        // self.left.unwrap().t < self.right.unwrap().t {
                        HitDirection::Left
                    } else {
                        HitDirection::Right
                    }
                }
            }
        } else {
            HitDirection::Miss
        }
    }
}
impl Node {
    pub fn add(&mut self, values: HitableList) {
        //   let target_node = if *value < self.aabb_box {
        //       &mut self.left
        //   } else {
        //       &mut self.right
        //   };
        //   match target_node {
        //       &mut Some(ref mut sub) => sub.new(value),
        //       &mut None => {
        //           let node = Node {
        //               aabb_box: value,
        //               left: None,
        //               right: None,
        //           };
        //           let boxed = Some(Box::new(node));
        //           *target_node = boxed;
        //       }
        //   }
    }
}
