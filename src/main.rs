// extern crate num;
extern crate image;

use std::fs::File;
use std::path::Path;

use num::complex::Complex;

fn main() {
    let image_x = 200;
    let image_y = 100;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(image_x, image_y);

    // Iterate over the coordiantes and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = x as f32 / image_x as f32;
        let g = (image_y - 1 - y) as f32 / image_y as f32;
        let b = 0.2;
        let base = 255.99;
        *pixel = image::Rgba([(base * r) as u8, (base * g) as u8, (base * b) as u8, 0]);
    }
    // Save the image as “fractal.png”
    let ref mut fout = File::create(&Path::new("fractal.ppm")).unwrap();

    // We must indicate the image’s color type and what format to save as
    let _ = image::ImageRgba8(imgbuf).save(fout, image::PPM);
}
