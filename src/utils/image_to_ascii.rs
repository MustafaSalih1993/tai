use crate::common::get_greyscale_img;
use crate::common::get_luminance;
use crate::setting::Settings;
use crate::utils::floyd_dither::floyd_dither;

pub fn img_to_ascii(x: Settings) {
    let mut img = if let Some(val) = get_greyscale_img(x) {
        val
    } else {
        return println!("Error: File Not Fount OR File Type Not Supported!");
    };

    // TODO: Move this table to Settings module as default, and accept a character array from the user.
    let table = [
        ' ', '.', ',', ':', ';', '\'', '"', '<', '>', 'i', '!', '(', ')', '[', ']', '(', ')', '{',
        '}', '*', '8', 'B', '%', '$', '#', '@',
    ];

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
