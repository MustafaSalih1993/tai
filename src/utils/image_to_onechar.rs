use crate::common::*;
use crate::setting::Settings;

//  will make the image to ONLY black and white
//  by converting the the "grays" to black or white based on the scale.
// source: https://en.wikipedia.org/wiki/Thresholding_(image_processing)

pub fn img_to_onechar(config: Settings) {
    let mut img = get_greyscale_img(&config).unwrap();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let mut pixel = img.get_pixel_mut(x, y).0;
            threshold_pixel(&mut pixel, 128);
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

fn threshold_pixel(pixel: &mut [u8; 3], scale: u8) {
    for i in 0..3 {
        if pixel[i] < scale {
            pixel[i] = 0;
        } else {
            pixel[i] = 255;
        }
    }
}
