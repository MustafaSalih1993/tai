use crate::common::{get_luminance, greyscaled};
use crate::config::Config;

//  will make the image to ONLY black and white
//  by converting the the "grays" to black or white based on the scale.
// source: https://en.wikipedia.org/wiki/Thresholding_(image_processing)

pub fn img_to_onechar(config: Config) {
    let mut img = greyscaled(&config).unwrap();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let mut pixel = img.get_pixel_mut(x, y).0;
            threshold_pixel(&mut pixel, config.threshold);
            if get_luminance(pixel[0], pixel[1], pixel[2]) > 128.0 {
                print!("{}", config.onechar) //■
            } else {
                print!(" ")
            };
        }
        println!();
    }
    println!();
}
// modify pixel values
fn threshold_pixel(pixel: &mut [u8; 4], scale: u8) {
    let mut scale = scale;
    for val in pixel.iter_mut().take(3) {
        if val < &mut scale {
            *val = 0;
        } else {
            *val = 255;
        }
    }
}
