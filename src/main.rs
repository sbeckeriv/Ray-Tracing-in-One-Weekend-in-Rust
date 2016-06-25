extern crate image;
extern crate nalgebra ;
extern crate simple_parallel;
extern crate rand;
extern crate flame;
use rand::distributions::{IndependentSample, Range};
use nalgebra::Vec3;
use std::sync::Arc;
use std::fs::File;
use std::path::Path;
mod utils;
use utils::unit_vector;
mod ray;
use ray::Ray;
mod objects;
mod worlds;
use worlds::{three_world, corner_world, random_world};
use objects::{Hitable, BVHFindHit, HitableList, sphere};
use objects::bvh::Node;
use objects::sphere::{MovingSphere, Sphere};
mod camera;
use camera::{normal_cam, normal_cam2, book_cam, head_on_cam};
mod material;
use std::fs;

fn main() {
    flame::start("main");
    let scene = 20;
    let image_x = 200;
    let image_y = 100;
    let frame_count = 1;
    let frame_count_string = format!("{}", frame_count);
    let ns = 200;
    let (world, old_world) = corner_world();
    fs::create_dir_all(format!("move/{}", scene)).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
    let mut world_index = 0;
    for w in [world, old_world].iter() {
        for i in 0..frame_count {
            world_index += 1;
            //let x_off = i as f32 / 10.0;
            let camera = normal_cam(&image_x, &image_y, 1.0, 2.0, 4.0);
            let random_index = Range::new(0.0, 1.0);
            // Create a new ImgBuf with width: imgx and height: imgy
            let mut imgbuf: image::RgbImage = image::ImageBuffer::new(image_x, image_y);
            let mut pool = simple_parallel::Pool::new(1);
            pool.for_(imgbuf.enumerate_pixels_mut(), |(x, y, pixel)| {
                let mut rng = rand::thread_rng();
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..ns {
                    let rand_x = random_index.ind_sample(&mut rng);
                    let rand_y = random_index.ind_sample(&mut rng);
                    let u = (x as f32 + rand_x) / image_x as f32;
                    let v = ((image_y - 1 - y) as f32 + rand_y) / image_y as f32;
                    let mut ray = camera.get_ray(&u, &v);
                    if x == 150 && y == 48 {
                        ray.debug = true;
                    }
                    col = col + color(&ray, &w, 0);
                }
                let base = 255.99;
                col = col / ns as f32;
                col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
                *pixel = image::Rgb([(base * col.x) as u8,
                                     (base * col.y) as u8,
                                     (base * col.z) as u8]);
            });
            // let jpg_file  = format!("move/scene_{}_{}.jpg", scene, i);
            // let ref mut fout = File::create(&Path::new(&jpg_file)).unwrap();
            // let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::JPEG);
            let ppm_file = format!("move/{}/{}_scene_{:03$}.ppm",
                                   scene,
                                   world_index,
                                   i,
                                   frame_count_string.len());

            let ref mut fout = File::create(&Path::new(&ppm_file)).unwrap();
            let _ = image::ImageRgb8(imgbuf.clone()).save(fout, image::PPM);
            println!("done {}", ppm_file);
        }
    }

    flame::end("main");
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();

}

fn color(ray: &Ray, world: &Arc<BVHFindHit>, depth: usize) -> Vec3<f32> {
    let hit_list: HitableList = world.find_hit(ray, 0.001, std::f32::MAX);
    match hit_list.hit(ray, &0.001, &std::f32::MAX) {
        Some((t, material)) => {
            if depth < 50 {
                let result = material.scatter(ray, &t);
                if ray.debug {
                    println!("scatters {:?}", result)
                }
                match result {
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
