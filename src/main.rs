extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::ThreadRng;
use na::Vec3;
use std::f32;
use std::rc::Rc;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
mod utils;
use utils::unit_vector;
mod ray;
use ray::Ray;
mod objects;
use objects::{HitableList, Sphere};
mod camera;
use camera::Camera;
mod material;

fn color(ray: &Ray, world: &HitableList, depth: usize, rand: &mut ThreadRng) -> Vec3<f32> {
    match world.hit(ray, &0.001, &std::f32::MAX) {
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
            let direction: Vec3<f32> = unit_vector(&ray.direction);
            let t = 0.5 * (direction.y + 1.0);
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn normal_cam(image_x: &u32, image_y: &u32) -> Camera {
    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    Camera::new_positionable(Vec3::new(0.0 - 2.0, 2.0, 1.0),
    Vec3::new(0.0, 0.0, 0.0 - 1.0),
    Vec3::new(0.0, 1.0, 0.0),
    90.0,
    *image_x as f32 / *image_y as f32)
}
fn test_cam(image_x: &u32, image_y: &u32) -> Camera {
    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    Camera::new_positionable(Vec3::new(0.0 - 2.0, 2.0, 1.0),
    Vec3::new(0.0, 0.0, 0.0 - 1.0),
    Vec3::new(0.0, 1.0, 0.0),
    90.0,
    *image_x as f32 / *image_y as f32)
}

fn world() -> HitableList {
    let mat1 = Rc::new(material::Lambertian::new(Vec3::new(0.8, 0.3, 0.3)));
    let mat2 = Rc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let metal1 = Rc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let die1 = Rc::new(material::Dielectric::new(1.5));

    let mut world = HitableList::new();
    world.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0 - 1.0), 0.5, mat1.clone()));
    world.push(Sphere::new(Vec3::new(0.0, 0.0 - 100.5, 0.0 - 1.0), 100.0, mat2.clone()));
    world.push(Sphere::new(Vec3::new(1.0, 0.0, 0.0 - 1.0), 0.5, metal1.clone()));
    world.push(Sphere::new(Vec3::new(0.0 - 1.0, 0.0, 0.0 - 1.0), 0.5, die1.clone()));
    world
}

fn world2() -> HitableList {
    // camera red blue balls
    let r = (f32::consts::PI / 4.0).cos();
    let mat1 = Rc::new(material::Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    let mat2 = Rc::new(material::Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));
    let mut world = HitableList::new();
    world.push(Sphere::new(Vec3::new(r * (0.0 - 1.0), 0.0, 0.0 - 1.0), r, mat1.clone()));
    world.push(Sphere::new(Vec3::new(r, 0.0, 0.0 - 1.0), r, mat2.clone()));
    world
}

fn main() {
    let image_x = 200;
    let image_y = 200;
    let mut rng = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let ns = 100;

    let camera = normal_cam(&image_x, &image_y);
    let mut world = world();

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_x, image_y);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
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
        *pixel = image::Rgb([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8]);
    }
    let ref mut fout = File::create(&Path::new("fractal.jpeg")).unwrap();
    let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::JPEG);

    let ref mut fout = File::create(&Path::new("fractal.ppm")).unwrap();
    let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::PPM);
    {
        let ref mut fout = File::create(&Path::new("home_fractal.ppm")).unwrap();
        let string = format!("P3\n {} {}\n255\n", image_x, image_y);
        fout.write(string.as_bytes());
        for pixel in imgbuf.pixels() {
            let string = format!("{} {} {}\n", pixel.data[0], pixel.data[1], pixel.data[2]);
            fout.write(string.as_bytes());
        }
    }

}
