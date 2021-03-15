use crate::common::get_luminance;
use crate::common::greyscaled;
use crate::config::Config;
use crate::utils::floyd_dither::floyd_dither;

// TODO: this is ugly, also not forgetting about the fact that it prints a char for every pixel i need to fix this soon!
// img_to_ascii converts to ascii,numbers,blocks
pub fn img_to_ascii(config: Config, table: &[char]) {
    let mut img = if let Some(val) = greyscaled(&config) {
        val
    } else {
        return eprintln!("Error: File Not Fount OR File Type Not Supported!");
    };

    if config.dither {
        floyd_dither(&mut img);
    };
    // loop on every pixel in y and x of the image and calculate the luminance.
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, _] = pixel.0;
            select_char(&table, get_luminance(r, g, b));
        }
        println!();
    }
    println!();
}

// decide which character to choose from the table(array);
fn select_char(table: &[char], lumi: f32) {
    print!(
        "{}",
        table[((lumi / 255.0) * (table.len() - 1) as f32) as usize]
    );
}
