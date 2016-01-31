extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use std::fs::File;
use std::path::Path;
use nalgebra::Dot;
mod ray;
use ray::Ray;
fn hit_sphere(center: &Vec3<f32>, radius: f32, ray: &Ray) -> bool {
    let origin = ray.origin;
    let direction = ray.direction;
    let oc = origin - *center;
    let a = direction.dot(&direction);
    let b = 2.0 * oc.dot(&direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminate = b * b - 4.0 * a * c;
    //println!("{:?} {},{},{}",discriminate,a,b,c);
    discriminate > 0.0
}

fn color(ray: &ray::Ray) -> Vec3<f32> {
    let direction: Vec3<f32> = ray.direction;
    let sphere = Vec3::new(0.0,0.0,0.0-1.0);
    if hit_sphere(&sphere, 0.5, ray){
        println!("in hit sphsere");
        Vec3::new(1.0,0.0,0.0)
    }else{
        let t = 0.5 * (direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    let image_x = 200;
    let image_y = 100;
    let lower_left_corner = Vec3::new(0.0 - 2.0, 0.0 - 1.0, 0.0 - 1.0);
    let horizon = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_x, image_y);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let u = x as f32 / image_x as f32;
        let v = (image_y - 1 - y) as f32 / image_y as f32;
        let ray = Ray::new(origin, lower_left_corner + horizon * u + vertical * v);
        let col = color(&ray);
        let base = 255.99;
        *pixel = image::Rgba([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8, 0]);
    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.ppm")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PPM);
}
