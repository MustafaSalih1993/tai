use crate::arguments::config::Config;
use crate::operations::otsu_threshold::OtsuThreshold;
use crate::utils::get_luma_buffer;
use image::Luma;

//  will make the image to ONLY black and white
//  by converting the the "grays" to black or white based on the scale.
// source: https://en.wikipedia.org/wiki/Thresholding_(image_processing)
// below we are using Otsu's thresholding which is automatically finds
// the best threshold value
// https://en.wikipedia.org/wiki/Otsu%27s_method
pub fn img_to_onechar(config: Config) {
    let mut img: image::ImageBuffer<image::Luma<u8>, Vec<u8>> = match get_luma_buffer(&config) {
        Some(img) => img,
        None => return,
    };
    img.threshold();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            if *pixel == Luma([255]) {
                print!("{}", config.onechar);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}
