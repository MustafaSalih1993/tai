use crate::common::get_greyscale_img;
use crate::common::get_luminance;
use crate::setting::{Settings, Style};
use crate::utils::{floyd_dither::floyd_dither, image_to_onechar::img_to_onechar};

pub fn img_to_ascii(config: Settings) {
    let mut img = if let Some(val) = get_greyscale_img(&config) {
        val
    } else {
        return println!("Error: File Not Fount OR File Type Not Supported!");
    };

    // FIXME IM UGLY.
    let table = match config.style {
        Style::OneChar => {
            img_to_onechar(config);
            return;
        }
        Style::Ascii => vec![
            ' ', '.', ',', ':', ';', '\'', '"', '<', '>', 'i', '!', '(', ')', '[', ']', '(', ')',
            '{', '}', '*', '8', 'B', '%', '$', '#', '@',
        ],
        Style::Numbers => vec![' ', '2', '7', '4', '1', '3', '9', '8', '5', '6', '0'],
        Style::Blocks => vec![' ', '░', '▒', '▓', '█'],
    };

    //TODO: make this optional!
    floyd_dither(&mut img);
    // loop on every pixel in y and x of the image and calculate the luminance.
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let [r, g, b] = pixel.0;
            select_char(&table, get_luminance(r, g, b));
        }
        println!();
    }
    println!();
}

// calculate and select a char from the table
fn select_char(table: &[char], lumi: f32) {
    print!(
        "{}",
        table[((lumi / 255.0) * (table.len() - 1) as f32) as usize]
    );
}
