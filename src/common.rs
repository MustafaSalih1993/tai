use crate::setting::Settings;
use image::{GenericImageView, ImageBuffer};

// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// calculate RGB values to get brightness of the pixel
pub fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    r + g + b
}

// this will resize the image and turn it into a greyscaled image(black, gray, and white);
pub fn get_greyscale_img(config: &Settings) -> Option<ImageBuffer<image::Rgb<u8>, Vec<u8>>> {
    let img = if let Ok(val) = image::open(&config.image_file) {
        val
    } else {
        return None;
    };

    //converting the image to greyscale
    let img = img
        .resize_exact(
            (img.width() / config.image_size) as u32,
            (img.height() / (config.image_size * 2) as u32) as u32,
            image::imageops::FilterType::Nearest,
        )
        .grayscale()
        .to_rgb8();

    Some(img)
}
