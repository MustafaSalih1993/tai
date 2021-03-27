use image::{GenericImageView, RgbaImage};

use crate::config::config::Config;

// luminance formula credits: https://stackoverflow.com/a/596243
// >>> Luminance = 0.2126*R + 0.7152*G + 0.0722*B <<<
// calculate RGB values to get luminance of the pixel
pub fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    let r = 0.2126 * (r as f32);
    let g = 0.7152 * (g as f32);
    let b = 0.0722 * (b as f32);
    r + g + b
}

// colorize a character
pub fn colorize(rgb: &[u8; 3], ch: char, bg_fg: u8) -> String {
    let prefix = format!("\x1B[{};2;{};{};{}m", bg_fg, rgb[0], rgb[1], rgb[2]);
    let postfix = "\x1B[0m";
    format!("{}{}{}", prefix, ch, postfix)
}

// this will open the image path,
// and resize the image and turn it into image buffer;
pub fn process_image(config: &Config) -> Option<RgbaImage> {
    let img = if let Ok(image) = image::open(&config.image_file) {
        image
    } else {
        eprintln!("Image path is not correct, OR image format is not supported!");
        return None;
    };
    let width = ((img.width() / config.scale) / 2) as u32;
    let height = ((img.height() / config.scale) / 4) as u32;
    let img = img.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    let img = if config.colored {
        img.to_rgba8()
    } else {
        img.grayscale().to_rgba8()
    };
    Some(img)
}

// program help message
pub fn print_usage() {
    println!("USAGE: tai [OPTIONS] [IMAGE_FILE]");
    println!();
    println!("OPTIONS: ");
    println!("\t -h | --help\t\t Show this help message");
    println!("\t -d | --dither\t\t enables image dithering");
    println!("\t -D | --dither-scale\t used with \"-d\" option, controls the scale number for the dithering, default to 16");
    println!("\t -s | --scale\t\t Followed by a number to Resize the output (lower number means bigger output) default to 2");
    println!("\t -t | --threshold\t Followed by a number (between 1 255) to select the threshold value,\n\
\t\t\t\t default to 128");
    println!(
        "\t -S | --style\t\t Followed by one of: {{ascii, numbers, blocks, onechar, braille}}, default to \"braille\""
    );
    println!("\t      --onechar\t\t Followed by a character, This will modify the default character used by (-S onechar)\n");
    println!("\t      --colored\t\t Will return true colored(RGB) art. ");
    println!("\t      --background\t Will apply the colors on the \"background\" of the characters instead of coloring the foreground.");
    println!("\t      --sleep\t\t Followed by number, controls the sleep delay(milli seconds) between animation frames. default to 100");
    println!("\t      --table\t\t Make a custom ascii table,(works only with \"ascii\" Style) seperated by ','\n\
\t\t\t\t ex: tai -S ascii --table \" ,.,:,x,@\" image.png");
    println!("\t -v | --version\t\t Print tai's Version and exit!.");
}
