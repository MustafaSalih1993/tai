use crate::config::Config;
use image::{GenericImageView, RgbaImage};

// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// calculate RGB values to get luminance of the pixel
pub fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    r + g + b
}

// this will open the image path,
// and resize the image and turn it into a greyscaled image(black, gray, and white);
pub fn greyscaled(config: &Config) -> Option<RgbaImage> {
    // checking if the image path is valid
    let img = match image::open(&config.image_file) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("common.greyscaled: Failed to open the image path!");
            return None;
        }
    };
    //  converting the image to greyscale and resizing it
    // (the resizing here is sucks i figured out by testing, maybe
    // i will change as soon as i fix the "image to ascii" algorithm!)
    let img = img
        .resize_exact(
            (img.width() / config.scale) as u32,
            (img.height() / (config.scale * 2) as u32) as u32,
            image::imageops::FilterType::Nearest,
        )
        .grayscale()
        .to_rgba8();

    Some(img)
}

// program help message
pub fn print_usage(program_name: String) {
    println!("USAGE: {} [OPTIONS] [IMAGE_FILE]", program_name);
    println!();
    println!("OPTIONS: ");
    println!("\t -h | --help\t\t Show this help message");
    println!("\t -d | --dither\t\t enables image dithering");
    println!("\t -s | --scale\t\t Followed by a number to Resize the output (lower number means bigger output) default to 2");
    println!("\t -t | --threshold\t Followed by a number (between 1 255) to select the threshold value,\n\
\t\t\t\t it controls black/white. default to 128, works only with \"onechar\" and \"braille\" styles");
    println!(
        "\t -S | --style\t\t Followed by one of: {{ascii, numbers, blocks, onechar, braille}}, default to \"braille\""
    );
    println!("\t      --onechar\t\t Followed by a character, This will modify the default character used by (-S onechar)");
    println!(
        "\t -v | --version\t\t Print {}'s Version and exit!.",
        program_name
    );
}
