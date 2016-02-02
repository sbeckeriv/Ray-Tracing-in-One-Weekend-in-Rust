extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, ThreadRng};
use na::Vec3;
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use nalgebra::Dot;
mod ray;
use ray::Ray;
mod objects;
use objects::{HitableList, Sphere};
mod camera;
use camera::Camera;
mod material;
use material::{Metal, Lambertian};

fn random_in_unit_sphere(rand: &mut ThreadRng) -> Vec3<f32> {
    let random_index = Range::new(0.0, 1.0);
    let mut p: Vec3<f32>;
    let minus_vec = Vec3::new(1.0, 1.0, 1.0);
    loop {
        p = Vec3::new(random_index.ind_sample(rand),
        random_index.ind_sample(rand),
        random_index.ind_sample(rand)) * 2.0 - minus_vec;
        if p.dot(&p) < 1.0 {
            break;
        }
    }
    p
}

fn color(ray: &Ray, world: &HitableList, depth: usize, rand: &mut ThreadRng) -> Vec3<f32> {
    let direction: Vec3<f32> = ray.direction;
    let sphere = Vec3::new(0.0, 0.0, 0.0 - 1.0);
    match world.hit(ray, &0.0, &std::f32::MAX) {
        Some((t, material)) => {
            if depth < 50 {
                match material.scatter(ray, &t) {
                    Some((attenuation, scattered)) => {
                        attenuation * color(&scattered, world, depth + 1, rand)
                    }
                    None => Vec3::new(0.0, 0.0, 0.0),
                }
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let t = 0.5 * (direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    let image_x = 200;
    let image_y = 100;
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let ns = 100;

    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(origin, lower_left_corner, vertical, horizon);
    let mat1 = Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));
    let mat2 = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));

    let metal1 = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2)));
    let metal2 = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8)));
    let mut world = HitableList::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0 - 1.0), 0.5, mat1.clone()));
    world.push(Sphere::new(Vec3::new(0.0, 0.0 - 100.5, 0.0 - 1.0), 100.0, mat2.clone()));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, 0.0 - 1.0), 0.5, metal1.clone()));
    world.push(Sphere::new(Vec3::new(0.0-1.0, 0.0, 0.0 - 1.0), 0.5, metal2.clone()));
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_x, image_y);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for i in 0..ns {
            let rand_x = random_index.ind_sample(&mut rng);
            let rand_y = random_index.ind_sample(&mut rng);
            let u = (x as f32 + rand_x) / image_x as f32;
            let v = ((image_y - 1 - y) as f32 + rand_y) / image_y as f32;
            let ray = camera.get_ray(&u, &v);
            col = col + color(&ray, &world, 0, &mut rng);
        }
        let base = 255.99;
        col = col / ns as f32;
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        *pixel = image::Rgba([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8, 0]);
    }
    let ref mut fout = File::create(&Path::new("fractal.jpeg")).unwrap();
    let _ = image::ImageRgba8(imgbuf.clone()).save(fout, image::JPEG);

    let ref mut fout = File::create(&Path::new("fractal.ppm")).unwrap();
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PPM);
}
