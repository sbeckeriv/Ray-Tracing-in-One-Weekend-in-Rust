extern crate image;
extern crate nalgebra;
extern crate nalgebra as na;
use na::Vec3;
use std::fs::File;
use std::path::Path;

fn main() {
    let image_x = 200;
    let image_y = 100;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_x, image_y);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let col = Vec3::new(x as f32 / image_x as f32,
                            (image_y - 1 - y) as f32 / image_y as f32,
                            0.2);
        let base = 255.99;
        *pixel = image::Rgba([(base * col.x) as u8, (base * col.y) as u8, (base * col.z) as u8, 0]);
    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.ppm")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PPM);
}
