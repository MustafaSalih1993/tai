use crate::arguments::config::Config;
use crate::operations::dither::Dither;
use crate::utils::{colorize, get_luminance, open_and_resize, resize};
use image::{gif::GifDecoder, AnimationDecoder, DynamicImage, RgbaImage};
use std::{fs::File, thread::sleep, time::Duration};

/* STATIC IMAGES

algorithm for static images work this way:
    - open the image buffer
    - loop on the image buffer by 2x2 chuncks
    - calculate the luminance of the 2x2 chunck and get the average luminance
    - based on the luminance average select a character from the ascii table
    - print the selected character
*/

/* ANIMATED IMAGES

algorithm for animated images work this way:
    - open the image frames
    - convert each frame to ascii(like static image)
    - return array of the processed frames
    - loop into the array of frames and print it to stdout
*/

// img_to_ascii converts to ascii,numbers,blocks
pub fn img_to_ascii(config: Config, table: &[char]) {
    if config.image_file.ends_with(".gif") {
        print_animated_image(&config, table);
    } else {
        print_static_image(&config, table);
    }
}

// this function will loop into a small chunck of pixels (2*2) and return a string containing a character
fn get_char(img: &RgbaImage, config: &Config, table: &[char], x: u32, y: u32) -> String {
    let mut sum = 0.0;
    let mut count = 0.0;
    for iy in y..y + 2 {
        for ix in x..x + 2 {
            let [red, green, blue, _] = img.get_pixel(ix, iy).0;
            let lumi = get_luminance(red, green, blue);
            sum += lumi;
            count += 1.0;
        }
    }
    let lumi_avg = sum / count;
    let cha = table[(lumi_avg / 255.0 * ((table.len() - 1) as f32)) as usize];
    let cha = if config.colored {
        let [red, green, blue, _] = img.get_pixel(x, y).0;
        colorize(&[red, green, blue], cha, config.background)
    } else {
        format!("{}", cha)
    };
    cha
}
// process a static image
fn print_static_image(config: &Config, table: &[char]) {
    let mut img = match open_and_resize(config) {
        Some(img) => img,
        None => return,
    };

    if config.dither {
        img.dither(config.dither_scale);
    };

    for y in (0..img.height() - 2).step_by(2) {
        for x in (0..img.width() - 2).step_by(2) {
            let ch = get_char(&img, config, table, x, y);
            print!("{}", ch);
        }
        println!();
    }
    println!();
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
        let img = DynamicImage::ImageRgba8(frame.buffer().clone());
        let mut img = resize(img, &config);
        if config.dither {
            img.dither(config.dither_scale);
        }

        let translated_frame = translate_frame(&img, &config, table);
        // this code -> \x1B[r <- will seek/save the cursor position to the start of the art
        // read about control characters: https://en.wikipedia.org/wiki/Control_character
        // so for each frame will override the old one in stdout
        out_frames.push(format!("\x1B[r{}", translated_frame));
    }
    out_frames
}

// this function will convert the pixels into ascii chars, put it in a string and return it
fn translate_frame(img: &RgbaImage, config: &Config, table: &[char]) -> String {
    let mut out = String::new();
    for y in (0..img.height() - 2).step_by(2) {
        for x in (0..img.width() - 2).step_by(2) {
            let cha = get_char(&img, config, table, x, y);
            out.push_str(&cha);
        }
        out.push('\n');
    }
    out
}
