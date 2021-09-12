use crate::arguments::config::Config;
use image::{DynamicImage, GenericImageView, GrayImage, RgbaImage};

// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// calculate RGB values to get luminance of the pixel
pub fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    r + g + b
}

// colorize a character by surrounding it with true  term colors
pub fn colorize(rgb: &[u8; 3], ch: char, bg_fg: u8) -> String {
    let prefix = format!("\x1B[{};2;{};{};{}m", bg_fg, rgb[0], rgb[1], rgb[2]);
    let postfix = "\x1B[0m";
    format!("{}{}{}", prefix, ch, postfix)
}

//rescale the image and convert to image buffer
pub fn open_and_resize(config: &Config) -> Option<RgbaImage> {
    let img = if let Ok(image) = image::open(&config.image_file) {
        image
    } else {
        eprintln!("Image path is not correct, OR image format is not supported!\n try -h | --help");
        return None;
    };
    let width = match config.original_size {
        true => img.width(),
        false => ((img.width() / config.scale) / 2) as u32,
    };
    let height = match config.original_size {
        true => img.height(),
        false => ((img.height() / config.scale) / 4) as u32,
    };
    let img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let img = if config.colored {
        img.into_rgba8()
    } else {
        img.grayscale().into_rgba8()
    };
    Some(img)
}

pub fn resize(img: DynamicImage, config: &Config) -> RgbaImage {
    let (width, height) = match config.original_size {
        false => {
            let width = ((img.width() / config.scale) / 2) as u32;
            let height = ((img.height() / config.scale) / 4) as u32;
            (width, height)
        }
        true => (img.width(), img.height()),
    };
    img.resize(width, height, image::imageops::FilterType::Lanczos3)
        .to_rgba8()
}

// this will open the image path,
// and resize the image and turn it into image buffer;
pub fn get_luma_buffer(config: &Config) -> Option<GrayImage> {
    let img = if let Ok(image) = image::open(&config.image_file) {
        image
    } else {
        eprintln!("Image path is not correct, OR image format is not supported!\n try -h | --help");
        return None;
    };
    let width = match config.original_size {
        true => img.width(),
        false => ((img.width() / config.scale) / 2) as u32,
    };
    let height = match config.original_size {
        true => img.height(),
        false => ((img.height() / config.scale) / 4) as u32,
    };
    let img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let img = img.to_luma8();
    Some(img)
}
