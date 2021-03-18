mod config;
mod operations;
mod utils;

use config::{Config, Style};
use operations::{
    image_to_ascii::img_to_ascii, image_to_braille::img_to_braille,
    image_to_onechar::img_to_onechar,
};
use std::env;

fn main() {
    let mut args = env::args();
    // parse args and return a valid config with defaults
    let config = match Config::new(&mut args) {
        Some(val) => val,
        None => return,
    };
    // matching the style givin.
    match config.style {
        Style::OneChar => {
            img_to_onechar(config);
        }
        Style::Braille => {
            img_to_braille(config);
        }
        Style::Ascii => {
            let table = vec![
                ' ', '.', ',', ':', ';', '\'', '"', '<', '>', 'i', '!', '(', ')', '[', ']', '(',
                ')', '{', '}', '*', '8', 'B', '%', '$', '#', '@',
            ];
            img_to_ascii(config, &table);
        }
        Style::Numbers => {
            let table = vec![' ', '2', '7', '4', '1', '3', '9', '8', '5', '6', '0'];
            img_to_ascii(config, &table);
        }
        Style::Blocks => {
            let table = vec![' ', '░', '▒', '▓', '█'];
            img_to_ascii(config, &table);
        }
    };
}
