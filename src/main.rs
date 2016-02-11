extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
extern crate rand;
extern crate simple_parallel;
use rand::distributions::{IndependentSample, Range};
use rand::ThreadRng;
use na::Vec3;
use std::f32;
use std::rc::Rc;
use std::sync::Arc;
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

fn color(ray: &Ray, world: &HitableList, depth: usize) -> Vec3<f32> {
    match world.hit(ray, &0.001, &std::f32::MAX) {
        Some((t, material)) => {
            if depth < 50 {
                match material.scatter(ray, &t) {
                    Some((attenuation, scattered)) => {
                        attenuation * color(&scattered, world, depth + 1)
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

fn normal_cam2(image_x: &u32, image_y: &u32) -> Camera {
    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let look_from = Vec3::new(3.0, 3.0, 2.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0 - 1.0);
    let distance = (look_from - look_at).len() as f32;
    let aperture = 2.0;
    Camera::new_focus(look_from,
                      look_at,
                      Vec3::new(0.0, 1.0, 0.0),
                      20.0,
                      *image_x as f32 / *image_y as f32,
                      aperture,
                      distance)
}

fn normal_cam(image_x: &u32, image_y: &u32) -> Camera {

    let look_from = Vec3::new(0.0-4.0, 3.5, 0.0-2.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0 - 1.0);
    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    Camera::new_positionable(look_from,
                             look_at,
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
/*
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
*/
fn random_world() -> HitableList {
    let mut rng = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let random_size_index = Range::new(0.03, 0.55);
    let mut world = HitableList::new();
    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(Vec3::new(0.0, (0.0 - 1000.0), 0.0), 1000.0, base_mat.clone()));
    let minus_vec = Vec3::new(4.0, 0.2, 0.0);
    for a in ((0 - 21)..22) {
        for b in ((0 - 11)..22) {
            let rand_size = random_size_index.ind_sample(&mut rng);
            let rand_mat = random_index.ind_sample(&mut rng);
            let center = Vec3::new(a as f32 + 0.9 * random_index.ind_sample(&mut rng),
            0.2,
            b as f32 * 0.9 * random_index.ind_sample(&mut rng));
            if (center - minus_vec).len() as f32 > 0.9 {
                if rand_mat < 0.8 {
                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(one, two, three)));
                    world.push(Sphere::new(center, rand_size, base_mat.clone()));
                } else if rand_mat < 0.95 {
                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let four = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let metal_vec = Vec3::new(0.5 * (1.0 + one), 0.5 * (1.0 + two), 0.5 * (1.0 + three));
                    let base_mat = Arc::new(material::Metal::new(
                            Vec3::new(0.5 * (1.0 + one), 0.5 * (1.0 + two), 0.5 * (1.0 + three))
                            , 0.5 * four));
                    world.push(Sphere::new(center, rand_size, base_mat.clone()));
                } else {
                    let base_mat = Arc::new(material::Dielectric::new(1.5));
                    world.push(Sphere::new(center, rand_size, base_mat.clone()));
                }
            }
        }
        //let die1 = Rc::new(material::Dielectric::new(1.5));
        //world.push(Sphere::new(Vec3::new(0.0, 1.0 , 0.0), 1.0, die1.clone()));

        //let lam1 = Rc::new(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
        //world.push(Sphere::new(Vec3::new(0.0-4.0, 1.0 , 0.0), 4.0, lam1.clone()));

        let metal1= Arc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5),0.0));
        world.push(Sphere::new(Vec3::new(4.0, 1.0 , 0.0), 1.0, metal1.clone()));
    }
    world
}
fn main() {
    let image_x = 400;
    let image_y = 400;
    let ns = 100;

    let camera_rc = Arc::new(normal_cam(&image_x, &image_y));
    let world_rc = Arc::new(random_world());
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
    let mut pool = simple_parallel::Pool::new(4);
    pool.for_(imgbuf.enumerate_pixels_mut(),|(x, y, pixel)|{
        let camera = camera_rc.clone();
        let world = world_rc.clone();
        let mut col = Vec3::new(0.0, 0.0, 0.0);

        let u = (x as f32 ) / image_x as f32;
        let v = ((image_y - 1 - y) as f32 ) / image_y as f32;
        let ray = camera.get_ray(&u, &v);
        col = col + color(&ray, &world, 0);
        let base = 255.99;
        //col = col / 100 as f32;
        col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
        *pixel = image::Rgb([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8]) ;
    });
    println!("done");
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
