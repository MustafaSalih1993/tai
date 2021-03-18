use crate::config::Config;
use crate::operations::floyd_dither::floyd_dither;
use crate::utils::{get_luminance, process_image};

// TODO: this is ugly, also not forgetting about the fact that it prints a char for every pixel i need to fix this soon!
// img_to_ascii converts to ascii,numbers,blocks
pub fn img_to_ascii(config: Config, table: &[char]) {
    let mut img = match process_image(&config) {
        Some(img) => img,
        None => return,
    };

    if config.dither {
        floyd_dither(&mut img);
    };

    // loop on every pixel in y and x of the image and calculate the luminance.
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, _] = pixel.0;
            let cha = select_char(&table, get_luminance(r, g, b));
            if config.colored {
                print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, cha);
            } else {
                print!("{}", cha);
            }
        }
        println!();
    }
    println!();
}

// decide which character to choose from the table(array);
fn select_char(table: &[char], lumi: f32) -> String {
    format!(
        "{}",
        table[((lumi / 255.0) * (table.len() - 1) as f32) as usize]
    )
}
