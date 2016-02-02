extern crate rand;
use rand::distributions::{IndependentSample, Range};
extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use objects::HitRecord;
use nalgebra::Dot;

pub trait Reflect{
    fn reflect(&self, v: &Vec3<f32>, n: &Vec3<f32>) -> Vec3<f32> {
        *v - (*n * v.dot(n) * 2.0)
    }
}

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

// I dont like that scatter needs reflect. I am unsure how to force a relationship here.
impl Reflect for Metal {}
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

pub struct Dielectric {
    ref_idx: f32,
}
impl Dielectric {
    pub fn new(ri: f32) -> Self {
        Dielectric { ref_idx: ri }
    }
    fn refract(&self, v: &Vec3<f32>, n: &Vec3<f32>, ni_over_nt: &f32) -> Option<Vec3<f32>> {
        let dt = v.dot(n);
        let discriminate = 1.0 - ni_over_nt * ni_over_nt * (1.0 * dt * dt);
        if discriminate > 0.0 {
            Some(((*v - *n * dt) * *ni_over_nt) - *n * discriminate.sqrt())
        } else {
            None
        }
    }
}
impl Reflect for Dielectric {}
impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)> {
        // let reflected = self.reflect(&r_in.direction, &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt) = if r_in.direction.dot(&rec.normal) > 0.0 {
            (rec.normal * (0.0 - 1.0), self.ref_idx)
        } else {
            (rec.normal, 1.0 / self.ref_idx)
        };
        if let Some(refracted) = self.refract(&r_in.direction, &outward_normal, &ni_over_nt) {
            Some((attenuation, Ray::new(rec.p, refracted)))
        } else {
            None
        }
    }
}
pub struct Lambertian {
    albedo: Vec3<f32>,
}
impl Lambertian {
    pub fn new(albedo: Vec3<f32>) -> Self {
        Lambertian { albedo: albedo }
    }
}
impl Scatter for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scarttered = Ray::new(rec.p, target - rec.p);
        Some((self.albedo, scarttered))
    }
}
