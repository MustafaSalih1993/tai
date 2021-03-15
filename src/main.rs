mod common;
mod config;
mod utils;

use config::{Config, Style};
use std::env;
use utils::{
    image_to_ascii::img_to_ascii, image_to_braille::img_to_braille,
    image_to_onechar::img_to_onechar,
};

fn main() {
    let mut args = env::args();
    // parse args and return a valid config with defaults
    let config = match Config::new(&mut args) {
        Some(val) => val,
        None => return eprintln!("Error: failed to parse config!"),
    };
    // matching the style givin.
    match config.style {
        Style::OneChar => {
            // only onchar based art, dont require a char table.
            img_to_onechar(config);
        }
        Style::Braille => {
            // this one generates characters internaly, doesnt require a char table.
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
