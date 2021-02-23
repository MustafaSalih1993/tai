mod common;
mod setting;
mod utils;

use setting::Settings;
use std::env;
use utils::image_to_ascii::img_to_ascii;

fn main() {
    let mut args = env::args();
    let settings = match Settings::new(&mut args) {
        Some(val) => val,
        None => return,
    };

    img_to_ascii(settings);
}

// TODO: MAKE THIS WORK
#[allow(unused)]
mod img_to_braille {
    use super::*;
    use common::*;

    pub fn to_braille(x: Settings) {
        todo!()
    }
}

// TODO: this will be a feature to show only 2 characters, will make the image to ONLY black and white
//       by converting the the "grays" to black or white based on the scale.

// source: https://en.wikipedia.org/wiki/Thresholding_(image_processing)
#[allow(unused)]
fn threshold_pixel(pixel: &mut [u8; 3], scale: u8) {
    for i in 0..3 {
        if pixel[i] < scale {
            pixel[i] = 0;
        } else {
            pixel[i] = 255;
        }
    }
}
