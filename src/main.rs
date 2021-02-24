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
