// base
// https://gist.github.com/aidanhs/5ac9088ca0f6bdd4a370
use na::Vec3;
use ray::Ray;
use utils::{ffmax, ffmin};
use objects::{HitableList, Hitable, HitableDirection};
use std::sync::Arc;
use std::cmp::Ordering;
use material;
#[derive(Debug)]
pub enum Hits {
    Hit,
    Miss,
    None,
}
#[derive(Debug)]
pub enum HitDirection {
    Left,
    Right,
    Miss,
    End,
    None,
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

    fn in_bounding_box(&self, point: Vec3<f32>) -> Hits {
        let local = self.bounding_box(0.0, 0.0);
        let results = local.0.x <= point.x && point.x <= local.1.x && local.0.y <= point.y &&
                      point.y <= local.1.y && local.0.z <= point.z &&
                      point.z <= local.1.z;
        // println!("In box {:?} {:?} {}", point, local, results);
        if results {
            Hits::Hit
        } else {
            Hits::Miss
        }
    }

    fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitDirection {
        // does the ray at this time hit?
        let point = ray.point_at_parameter(t_min);
        // println!("Point check {:?} ", point);
        match self.in_bounding_box(point) {
            Hits::Hit => {
                if self.left.is_none() && self.right.is_none() {
                    HitDirection::End
                } else {
                    let hit = match (self.left
                                         .as_ref()
                                         .map_or(Hits::None, |n| n.in_bounding_box(point)),
                                     self.right
                                         .as_ref()
                                         .map_or(Hits::None, |n| n.in_bounding_box(point))) {
                        (Hits::Miss, Hits::Miss) => HitDirection::Miss,
                        (Hits::None, Hits::None) => {
                            unreachable!("Both left and right can not be none here")
                        }
                        (left, Hits::Miss) | (left, Hits::None) => HitDirection::Left,
                        (Hits::None, right) | (Hits::Miss, right) => HitDirection::Right,
                        _ => {
                            HitDirection::Left
                            // both must hit
                            // match ray.closer(t_min,
                            //                 self.left
                            //                     .as_ref()
                            //                     .unwrap()
                            //                     .bounding_box(t_min, t_max),
                            //                 self.right
                            //                     .as_ref()
                            //                     .unwrap()
                            //                     .bounding_box(t_min, t_max)) {
                            //    HitableDirection::Left => HitDirection::Left,
                            //    HitableDirection::Right => HitDirection::Right,
                            //    _ => unreachable!("N is not closer to either?"),
                            // }
                        }
                    };
                    // println!("Hit result:: {:?}", hit);
                    hit
                }
            }
            _ => HitDirection::Miss,
        }
    }

    // http://stackoverflow.com/questions/4965335/how-to-print-binary-tree-diagram
    pub fn print(&self, prefix: String, is_tail: Option<()>) {
        let tail = is_tail.as_ref().map_or("  ├── ", |c| "  └── ");
        let hit_list = self.hitlist
                           .as_ref()
                           .map_or("".to_string(), |c| format!("{}", c.list.len()));
        println!("{}{} {:?} {}", prefix, tail, (self.min, self.max), hit_list);
        let child_tail = format!("{}{}", prefix, is_tail.map_or("  │   ", |c| "      "));
        if self.left.is_some() {
            self.left
                .as_ref()
                .map(|n| n.print(child_tail, self.right.as_ref().map_or(Some(()), |v| None)));
        }
        let child_tail = format!("{}{}", prefix, is_tail.map_or("  │   ", |c| "      "));
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
        let real_min = min.unwrap_or(Vec3::new(-16.0, -16.0, -16.0));
        let real_max = max.unwrap_or(Vec3::new(16.0, 16.0, 16.0));

        let mut head = Node {
            min: real_min,
            max: real_max,
            left: None,
            right: None,
            hitlist: None,
        };

        if list.len() > 5 && real_depth < 20 && head.min != head.max {
            let mid = (head.min + head.max) / 2.0;
            let (mut min_mid, mut max_mid) = Self::min_values(&head, &mid, real_depth);

            let (left, right): (Vec<Arc<Hitable>>, Vec<Arc<Hitable>>) =
                list.into_iter().partition(|n| {
                    match (n.overlaps_bounding_box(head.min, min_mid),
                           n.overlaps_bounding_box(max_mid, head.max)) {
                        (true, true) => {
                            match n.closer((head.min, min_mid), (max_mid, head.max)) {
                                HitableDirection::Left => true,
                                HitableDirection::Right => false,
                                _ => unreachable!("N is not closer to either?"),
                            }
                        }
                        (true, false) => true,
                        (false, true) => false,
                        (false, false) => {
                            unreachable!(format!("Object not in either left or right. This is a \
                                                  problem. Depth::{} n::{:?}",
                                                 real_depth,
                                                 n.bounding_box(0.0, 0.0)))
                        }
                    }

                });
            // dont run the tree if we have nothing to hit..clearly
            if left.len() > 0 {
                let left = Node::new(left,
                                     Some(head.min.clone()),
                                     Some(min_mid.clone()),
                                     Some(real_depth + 1));
                let left_boxed = Some(Box::new(left));
                head.left = left_boxed;
            }
            if right.len() > 0 {
                let right = Node::new(right,
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
        match self.hits(ray, t_min, t_max) {
            HitDirection::Left => self.left.as_ref().unwrap().find_hit(ray, t_min, t_max),
            HitDirection::Right => self.right.as_ref().unwrap().find_hit(ray, t_min, t_max),
            HitDirection::Miss | HitDirection::None => HitableList::new(),
            HitDirection::End => {
                println!("END {:?} {}",
                         self.bounding_box(0.0, 0.0),
                         self.hitlist.as_ref().unwrap().list.len());
                let mut hit_list = HitableList::new();
                for record in &self.hitlist.as_ref().unwrap().list {
                    hit_list.push(record.clone());
                }
                hit_list
            }
        }
    }
    pub fn min_values(node: &Node, mid: &Vec3<f32>, depth: i32) -> (Vec3<f32>, Vec3<f32>) {
        let mut min_mid = Vec3::new(node.max.x, node.max.y, node.max.z);
        let mut max_mid = Vec3::new(node.min.x, node.min.y, node.min.z);
        match depth % 3 {
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
        (min_mid, max_mid)
    }
}
