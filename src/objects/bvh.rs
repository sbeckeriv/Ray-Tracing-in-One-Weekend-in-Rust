// base
// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
use na::Vec3;
use ray::Ray;
use utils::{ffmax, ffmin};
use objects::{HitableList, Hitable};
use std::sync::Arc;
use std::cmp::Ordering;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use material;
pub enum HitDirection {
    Left,
    Right,
    Miss,
    End,
}
pub struct Node {
    pub min: Vec3<f32>,
    pub max: Vec3<f32>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
    pub hitlist: Option<HitableList>,
}

impl Node {
    fn bounding_box(&self, t0: f32, t1: f32) -> (Vec3<f32>, Vec3<f32>) {
        let one = self.min;
        let two = self.max;
        (one, two)
    }
    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitDirection {
        let mut hit = true;
        // Moved from aabb
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
            match (self.left.as_ref().map_or(HitDirection::Miss, |n| n.hits(ray, t_min, t_max)),
                   self.right.as_ref().map_or(HitDirection::Miss, |n| n.hits(ray, t_min, t_max))) {
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
    // http://stackoverflow.com/questions/4965335/how-to-print-binary-tree-diagram
    pub fn print(&self, prefix: String, is_tail: Option<()>) {
        let tail = is_tail.as_ref().map_or("├── ", |c| "└── ");
        let hit_list = self.hitlist
                           .as_ref()
                           .map_or("".to_string(), |c| format!("{}", c.list.len()));
        println!("{}{} {:?} {}", prefix, tail, (self.min, self.max), hit_list);
        let child_tail = format!("{}{}", prefix, is_tail.map_or("│   ", |c| "    "));
        if self.left.is_some() {
            self.left.as_ref().map(|n| n.print(child_tail, None));
        }
        let child_tail = format!("{}{}", prefix, is_tail.map_or("│   ", |c| "    "));
        if self.right.is_some() {
            self.right.as_ref().map(|n| n.print(child_tail, Some(())));
        }
    }
    pub fn new(list: Vec<Arc<Hitable>>,
               min: Option<Vec3<f32>>,
               max: Option<Vec3<f32>>,
               depth: Option<i32>)
               -> Node {

        let real_depth = depth.unwrap_or(1);
        let real_min = min.unwrap_or(Vec3::new(0.0, 0.0, 0.0));
        let real_max = max.unwrap_or(Vec3::new(1.0, 1.0, 1.0));

        let mut head = Node {
            min: real_min,
            max: real_max,
            left: None,
            right: None,
            hitlist: None,
        };

        println!("new node depth {:?} list {:?} :: {:?}",
                 real_depth,
                 list.len(),
                 (head.min, head.max));
        if list.len() > 10 && real_depth < 10 && head.min != head.max {
            let random_index = Range::new(0.0, 1.0);
            let mut rng = rand::thread_rng();
            println!("new split depth {:?}", real_depth);
            let mid = (head.min + head.max) / 2.0;
            let mut min_mid = Vec3::new(head.max.x, head.max.y, head.max.z);
            let mut max_mid = Vec3::new(head.min.x, head.min.y, head.min.z);
            match real_depth % 3 {
                1 => {
                    min_mid.x = mid.x;
                    max_mid.x = mid.x;
                }
                0 => {
                    min_mid.y = mid.y;
                    max_mid.y = mid.y;
                }
                _ => {
                    min_mid.z = mid.z;
                    max_mid.z = mid.z;
                }
            }
            let (even, odd): (Vec<Arc<Hitable>>, Vec<Arc<Hitable>>) =
                list.into_iter().partition(|n| {
                    match (n.overlaps_bounding_box(head.min, min_mid),
                           n.overlaps_bounding_box(max_mid, head.max)) {
                        (true, true) => {
                            let rand = random_index.ind_sample(&mut rng);

                            if rand > 0.4 {
                                false
                            } else {
                                true
                            }
                        }
                        (true, false) => true,
                        (false, true) => false,
                        _ => false,
                    }

                });
            // right left logic.
            println!("min:: {:?} mid::{:?} len:: {:?}",
                     head.min,
                     min_mid,
                     even.len());
            println!("max:: {:?} mid::{:?} len:: {:?}",
                     head.max,
                     max_mid,
                     odd.len());
            println!("mid:: {:?} ", mid);
            // dont run the tree if we have nothing to hit..clearly
            if odd.len() > 0 {
                let left = Node::new(odd,
                                     Some(head.min.clone()),
                                     Some(min_mid.clone()),
                                     Some(real_depth + 1));
                let left_boxed = Some(Box::new(left));
                head.left = left_boxed;
            }
            if even.len() > 0 {
                let right = Node::new(even,
                                      Some(max_mid.clone()),
                                      Some(head.max.clone()),
                                      Some(real_depth + 1));
                let right_boxed = Some(Box::new(right));
                head.right = right_boxed;
            }

        } else {
            let mut hitlist = HitableList::new();
            for record in list.clone() {
                hitlist.push(record);
            }
            head.hitlist = Some(hitlist);//lazy clone. might need lifetimes later.
        }
        head
    }
    pub fn find_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitableList {
        HitableList::new()
    }
}
