extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng, SeedableRng, StdRng};
use na::Vec3;
use std::fs::File;
use std::path::Path;
use nalgebra::Dot;
mod ray;
use ray::Ray;
mod objects;
use objects::{HitableList, Sphere, HitRecord};
mod camera;
use camera::Camera;

fn color(ray: &Ray, world: &HitableList) -> Vec3<f32> {
    let direction: Vec3<f32> = ray.direction;
    let sphere = Vec3::new(0.0, 0.0, 0.0 - 1.0);
    match world.hit(ray, &0.0, &std::f32::MAX) {
        Some(t) => {
            let n = t.normal;
            Vec3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5
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
    let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed);
    let random_index = Range::new(0.0, 1.0);
    let ns = 100;
    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let camera = Camera::new(origin, lower_left_corner,vertical, horizon);

    let mut world = HitableList::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0 - 1.0), 0.5));
    world.push(Sphere::new(Vec3::new(0.0, 0.0 - 100.5, 0.0 - 1.0), 100.0));
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_x, image_y);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = Vec3::new(0.0,0.0,0.0);
        for i in 0..ns{
            let rand_x = random_index.ind_sample(&mut rng);
            let rand_y = random_index.ind_sample(&mut rng);
            let u = (x as f32 + rand_x) / image_x as f32;
            let v = ((image_y - 1 - y) as f32+rand_y) / image_y as f32;
            let ray = camera.get_ray(&u, &v);
            col = col+ color(&ray, &world);
        }
        let base = 255.99;
        col = col / ns as f32;
        *pixel = image::Rgba([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8, 0]);
    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.jpeg")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(imgbuf).save(fout, image::JPEG);
}
