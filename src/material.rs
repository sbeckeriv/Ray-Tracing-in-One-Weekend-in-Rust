extern crate rand;
use rand::distributions::{IndependentSample, Range};
extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use ray::Ray;
use objects::HitRecord;
use nalgebra::Dot;
use utils::{random_in_unit_sphere, unit_vector};

pub trait Reflect{
    fn reflect(&self, v: &Vec3<f32>, n: &Vec3<f32>) -> Vec3<f32> {
        *v - (*n * v.dot(n) * 2.0)
    }
}

pub trait Scatter{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)>;
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
        let unit_directon = unit_vector(&r_in.direction);
        let reflected = self.reflect(&unit_directon, &rec.normal);
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
        let uv = unit_vector(&v);
        let dt = uv.dot(n);
        let discriminate = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminate > 0.0 {
            Some(((*v - *n * dt) * *ni_over_nt) - *n * discriminate.sqrt())
        } else {
            None
        }
    }
    fn schlick(&self, cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
impl Reflect for Dielectric {}
impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3<f32>, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if r_in.direction.dot(&rec.normal) > 0.0 {
            let cos = self.ref_idx * r_in.direction.dot(&rec.normal) / r_in.direction.len() as f32;
            (rec.normal * (0.0 - 1.0), self.ref_idx, cos)
        } else {
            let cos = (0.0 - 1.0) * r_in.direction.dot(&rec.normal) / r_in.direction.len() as f32;
            (rec.normal, (1.0 / self.ref_idx), cos)
        };
        let refracted = self.refract(&r_in.direction, &outward_normal, &ni_over_nt);
        let reflect_prob = if refracted.is_some() {
            self.schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        let mut rand = rand::thread_rng();
        let random_index = Range::new(0.0, 1.0);
        let random = random_index.ind_sample(&mut rand);
        if random < reflect_prob {
            let reflected = self.reflect(&r_in.direction, &rec.normal);
            Some((attenuation, Ray::new(rec.p, reflected)))
        } else {
            Some((attenuation, Ray::new(rec.p, refracted.unwrap())))
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
