extern crate rand;
use rand::distributions::{IndependentSample, Range};
extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use nalgebra::Dot;
pub fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
pub fn unit_vector(vec: &Vec3<f32>) -> Vec3<f32> {
    let len = vec.len() as f32;
    Vec3::new(vec.x / len, vec.y / len, vec.z / len)
}

pub fn random_unit_disk() -> Vec3<f32> {
    let mut rng = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let mut p: Vec3<f32>;
    let minus_vec = Vec3::new(1.0, 1.0, 0.0);
    loop {
        let one = random_index.ind_sample(&mut rng);
        let two = random_index.ind_sample(&mut rng);
        p = Vec3::new(one, two, 0.0) * 2.0 - minus_vec;
        if !(p.dot(&p) >= 1.0) {
            break;
        }
    }
    p
}

pub fn random_in_unit_sphere() -> Vec3<f32> {
    let mut rand = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let mut p: Vec3<f32>;
    let minus_vec = Vec3::new(1.0, 1.0, 1.0);
    loop {
        p = Vec3::new(random_index.ind_sample(&mut rand),
                      random_index.ind_sample(&mut rand),
                      random_index.ind_sample(&mut rand)) * 2.0 - minus_vec;
        if p.dot(&p) < 1.0 {
            break;
        }
    }
    p
}
