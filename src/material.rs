extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, SeedableRng, StdRng};
extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use objects::HitRecord;
use nalgebra::Dot;

pub trait Scatter{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)>;
}

fn random_in_unit_sphere() -> Vec3<f32> {
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
// todo make scatter a trait
pub struct Metal {
    albedo: Vec3<f32>,
    fuzz: f32,
}
impl Metal {
    fn reflect(&self, v: &Vec3<f32>, n: &Vec3<f32>) -> Vec3<f32> {
        *v - (*n * v.dot(n) * 2.0)
    }
    // move to util
    pub fn new(albedo: Vec3<f32>, fuzz: f32) -> Self {
        let clean_fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Metal {
            albedo: albedo,
            fuzz: clean_fuzz,
        }
    }
}
impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)> {
        let reflected = self.reflect(&r_in.direction, &rec.normal);
        let scarttered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        if scarttered.direction.dot(&rec.normal) > 0.0 {
            Some((self.albedo, scarttered))
        } else {
            None
        }
    }
}

pub struct Lambertian {
    albedo: Vec3<f32>,
}
impl Lambertian {
    // move to util
    pub fn new(albedo: Vec3<f32>) -> Self {
        Lambertian { albedo: albedo }
    }
}
impl Scatter for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scarttered = Ray::new(rec.p, target - rec.p);
        Some((self.albedo, scarttered))
    }
}
