use std::{fs::File, thread::sleep, time::Duration};

use image::{gif::GifDecoder, AnimationDecoder, DynamicImage, GenericImageView, RgbaImage};

use crate::config::config::Config;
use crate::operations::floyd_dither::floyd_dither;
use crate::utils::{colorize, get_luminance, process_image};

// TODO: this is ugly, also not forgetting about the fact that it prints a char for every pixel,
// SOLUTION: process the image buffer by "blocks" ex: 4x4 pixels per character.

// img_to_ascii converts to ascii,numbers,blocks
pub fn img_to_ascii(config: Config, table: &[char]) {
    if config.image_file.ends_with(".gif") {
        print_animated_image(&config, table);
    } else {
        print_static_image(&config, table);
    }
}

// decide which character to choose from the table(array);
fn select_char(table: &[char], lumi: f32) -> char {
    table[((lumi / 255.0) * (table.len() - 1) as f32) as usize]
}

fn print_static_image(config: &Config, table: &[char]) {
    // let mut img = match process_image(config) {
    //     Some(img) => img,
    //     None => return,
    // };

    let img = if let Ok(image) = image::open(&config.image_file) {
        image
    } else {
        return eprintln!("Image path is not correct, OR image format is not supported!");
    };
    let width = ((img.width() / config.scale) / 2) as u32;
    let height = ((img.height() / config.scale) / 4) as u32;
    let mut img = img
        .resize_exact(width, height, image::imageops::FilterType::Lanczos3)
        .to_rgba8();

    if config.dither {
        floyd_dither(&mut img);
    };

    // loop on every pixel in y and x of the image and calculate the luminance.
    for y in 0..img.height() - 4 {
        for x in 0..img.width() - 2 {
            loop_block(&img, config, table, x, y);
            // let [r, g, b, _] = img.get_pixel(x, y).0;
            // let cha = select_char(&table, get_luminance(r, g, b));
            // if config.colored {
            //     print!("{}", colorize(&[r, g, b], cha, config.background));
            // } else {
            //     print!("{}", cha);
            // }
        }
        println!();
    }
    println!();
}
fn loop_block(img: &RgbaImage, config: &Config, table: &[char], x: u32, y: u32) {
    let mut sum = 0.0;
    for iy in y..y + 4 {
        for ix in x..x + 2 {
            let [r, g, b, _] = img.get_pixel(ix, iy).0;
            sum += get_luminance(r, g, b);
        }
    }
    let lumi_avg = sum / 8.0;
    let cha = table[(lumi_avg / 255.0 * (table.len() as f32)) as usize];
    if config.colored {
        let [r, g, b, _] = img.get_pixel(x, y).0;
        print!("{}", colorize(&[r, g, b], cha, config.background));
    } else {
        print!("{}", cha);
    }
}
// this function will loop into frames converted to ascii
// and sleep between each frame
fn print_animated_image(config: &Config, table: &[char]) {
    let frames = get_animated_frames(config, table);
    loop {
        for frame in &frames {
            print!("{}", frame);
            sleep(Duration::from_millis(config.sleep))
        }
    }
}

// this function will open an animation file, decode it, and convert
// it's frames pixels into ascii, will return a vector containing a
// frames converted to ascii string
fn get_animated_frames(config: &Config, table: &[char]) -> Vec<String> {
    let mut out_frames = Vec::new(); // this is the return of this function
    let file_in = match File::open(&config.image_file) {
        Ok(file) => file,
        Err(_) => return out_frames,
    };
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder
        .into_frames()
        .collect_frames()
        .expect("error decoding gif");
    // pushing this ansi code to clear the screen in the start of the frames
    out_frames.push("\x1B[1J".to_string());

    for frame in frames {
        // prolly this is not efficient, need to read image crate docs more!
        let img = DynamicImage::from(DynamicImage::ImageRgba8(frame.buffer().clone()));
        let width = ((frame.buffer().width() / config.scale) / 2) as u32;
        let height = ((frame.buffer().height() / config.scale) / 4) as u32;
        let mut img = img
            .resize_exact(width, height, image::imageops::FilterType::Lanczos3)
            .to_rgba8();
        if config.dither {
            floyd_dither(&mut img);
        }
        let translated_frame = translate_frame(&img, &config, table);
        // this code will seek/save the cursor position to the start of the art
        // read about control characters: https://en.wikipedia.org/wiki/Control_character
        // so for each frame will override the old one in stdout
        out_frames.push(format!("\x1B[r{}", translated_frame));
    }
    out_frames
}

// this function will convert the pixels into ascii chars, put it in a string and return it
fn translate_frame(img: &RgbaImage, config: &Config, table: &[char]) -> String {
    let mut out = String::new();
    for y in 0..img.height() {
        for x in 0..img.width() {
            let [r, g, b, _] = img.get_pixel(x, y).0;
            let ch = select_char(&table, get_luminance(r, g, b));
            if config.colored {
                out.push_str(&colorize(&[r, g, b], ch, config.background));
            } else {
                out.push(ch);
            }
        }
        out.push('\n');
    }
    out
}
