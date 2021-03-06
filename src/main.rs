extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
extern crate simple_parallel;
extern crate rand;
use rand::distributions::{IndependentSample, Range};
use na::Vec3;
use std::sync::Arc;
use std::fs::File;
use std::path::Path;
mod utils;
use utils::unit_vector;
mod ray;
use ray::Ray;
mod objects;
use objects::{HitableList, sphere};
use objects::sphere::{MovingSphere, Sphere};
mod camera;
use camera::Camera;
mod material;
use std::fs;

fn main() {
    let scene = 16;
    let image_x = 200;
    let image_y = 100;
    let frame_count = 1;
    let frame_count_string = format!("{}", frame_count);
    let ns = 100;
    let world = random_world();

    println!("mkdir");
    fs::create_dir_all(format!("move/{}", scene)).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    for i in 0..frame_count {
        let x_off = i as f32 / 10.0;
        let camera = normal_cam(&image_x, &image_y, x_off, 0.0, 0.0);
        let random_index = Range::new(0.0, 1.0);
        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
        let mut pool = simple_parallel::Pool::new(8);
        pool.for_(imgbuf.enumerate_pixels_mut(), |(x, y, pixel)| {
            let mut rng = rand::thread_rng();
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let rand_x = random_index.ind_sample(&mut rng);
                let rand_y = random_index.ind_sample(&mut rng);
                let u = (x as f32 + rand_x) / image_x as f32;
                let v = ((image_y - 1 - y) as f32 + rand_y) / image_y as f32;
                let ray = camera.get_ray(&u, &v);
                col = col + color(&ray, &world, 0);
            }
            let base = 255.99;
            col = col / ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            *pixel = image::Rgb([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8]);
        });
        // let jpg_file  = format!("move/scene_{}_{}.jpg", scene, i);
        // let ref mut fout = File::create(&Path::new(&jpg_file)).unwrap();
        // let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::JPEG);
        let ppm_file = format!("move/{}/scene_{:02$}.ppm",
                               scene,
                               i,
                               frame_count_string.len());

        let ref mut fout = File::create(&Path::new(&ppm_file)).unwrap();
        let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::PPM);
        println!("done {}", ppm_file);
    }
}

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

fn normal_cam2(image_x: &u32,
               image_y: &u32,
               offset_x: f32,
               offset_y: f32,
               offset_z: f32)
    -> Camera {
        let look_from = Vec3::new(3.0 + offset_x, 3.0 + offset_y, 2.0 + offset_z);
        let look_at = Vec3::new(0.0, 0.0, 0.0 - 1.0);
        let distance = (look_from - look_at).len() as f32;
        let aperture = 0.0;
        Camera::new_focus(look_from,
                          look_at,
                          Vec3::new(0.0, 1.0, 0.0),
                          20.0,
                          *image_x as f32 / *image_y as f32,
                          aperture,
                          distance,
                          0.0,
                          0.0)
    }

fn head_on_cam(image_x: &u32,
               image_y: &u32,
               offset_x: f32,
               offset_y: f32,
               offset_z: f32)
    -> Camera {
        Camera::new_set()
    }

fn normal_cam(image_x: &u32, image_y: &u32, offset_x: f32, offset_y: f32, offset_z: f32) -> Camera {
    let look_from = Vec3::new(13.0, 2.0 + offset_y, 3.0 + offset_z);
    let look_at = Vec3::new(0.0 + offset_x , 0.0, 0.0 );

    let distance = 10.0;
    let aperture = 0.0;
    Camera::new_focus(look_from,
                      look_at,
                      Vec3::new(0.0, 1.0, 0.0),
                      20.0,
                      *image_x as f32 / *image_y as f32,
                      aperture,
                      distance,
                      0.0,
                      1.0)

}

fn three_world() -> HitableList {
    let mut world = HitableList::new();
    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, (0.0 - 100.5), 0.0), 100.0, base_mat.clone()));
    world.push(sphere.clone());


    let lam1 = Arc::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0 - 1.0), 0.5, lam1.clone()));
    world.push(sphere.clone());

    let metal1 = Arc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let sphere = Arc::new(Sphere::new(Vec3::new(1.0, 0.0, 0.0 - 1.0), 0.5, metal1.clone()));
    world.push(sphere.clone());

    let die1 = Arc::new(material::Dielectric::new(1.5));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0 - 1.0, 0.0, 0.0 - 1.0), 0.5, die1.clone()));
    world.push(sphere.clone());
    world
}

fn random_world() -> HitableList {
    let mut rng = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let random_size_index = Range::new(0.03, 0.55);
    let mut world = HitableList::new();
    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, (0.0 - 1000.0), 0.0),
    1000.0,
    base_mat.clone()));
    world.push(sphere.clone());
    let minus_vec = Vec3::new(4.0, 0.2, 0.0);
    for a in (0 - 11)..12 {
        for b in (0 - 11)..12 {
            let rand_size = random_size_index.ind_sample(&mut rng);
            let rand_mat = random_index.ind_sample(&mut rng);
            let center = Vec3::new(a as f32 + 0.9 * random_index.ind_sample(&mut rng),
            0.2,
            b as f32 * 0.9 * random_index.ind_sample(&mut rng));
            if (center - minus_vec).len() as f32 > 0.9 {
                let sphere: Arc<objects::Hitable> = if rand_mat < 0.1 {

                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(one, two, three)));
                    let center1 = center + Vec3::new(0.0, 0.5, 0.0);

                    Arc::new(MovingSphere::new(center,
                                               center1,
                                               rand_size,
                                               base_mat.clone(),
                                               0.0,
                                               1.0))
                } else if rand_mat < 0.8 {
                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(one, two, three)));
                    Arc::new(Sphere::new(center, rand_size, base_mat.clone()))
                } else if rand_mat < 0.95 {
                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let four = random_index.ind_sample(&mut rng) *
                        random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Metal::new(Vec3::new(0.5 * (1.0 + one),
                    0.5 * (1.0 + two),
                    0.5 * (1.0 + three)),
                    0.5 * four));
                    Arc::new(Sphere::new(center, rand_size, base_mat.clone()))
                } else {
                    let base_mat = Arc::new(material::Dielectric::new(1.5));
                    Arc::new(Sphere::new(center, rand_size, base_mat.clone()))
                };
                world.push(sphere.clone());
            }
        }
        let die1 = Arc::new(material::Dielectric::new(1.5));
        let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, die1.clone()));
        world.push(sphere.clone());

        let metal1 = Arc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
        let sphere = Arc::new(Sphere::new(Vec3::new(0.0 - 4.0, 1.0, 0.0), 1.0, die1.clone()));
        world.push(sphere.clone());

        let sphere = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, metal1.clone()));
        world.push(sphere.clone());
    }
    world
}
