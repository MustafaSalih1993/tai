use image::ImageBuffer;
// TODO: make the factor dynamic by the user to control how many colors will be inside.
// This algorithm to make a dithered image.
// source : https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering

pub fn floyd_dither(img: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    // TODO: make the factor dynamic by the user!.
    // this will control the colors in the image.
    let factor = 16.0;

    for y in 0..img.height() - 1 {
        for x in 1..img.width() - 1 {
            let old_rgb: [u8; 3] = img.get_pixel(x, y).0;
            let new_rgb: [u8; 3] = find_closest_color(old_rgb, factor);
            // replace the pixel colors after calculation.
            for i in 0..3 {
                img.get_pixel_mut(x, y).0[i] = new_rgb[i];
            }

            let mut pixel = img.get_pixel_mut(x, y).0;
            pixel[0] = new_rgb[0];
            pixel[1] = new_rgb[1];
            pixel[2] = new_rgb[2];

            let err_r: f32 = old_rgb[0] as f32 - new_rgb[0] as f32;
            let err_g: f32 = old_rgb[1] as f32 - new_rgb[1] as f32;
            let err_b: f32 = old_rgb[2] as f32 - new_rgb[2] as f32;
            let err_pixel = [err_r, err_g, err_b];

            calculate_and_assign_pixel(img, (x + 1, y), err_pixel, 7.0);
            calculate_and_assign_pixel(img, (x - 1, y + 1), err_pixel, 3.0);
            calculate_and_assign_pixel(img, (x, y + 1), err_pixel, 5.0);
            calculate_and_assign_pixel(img, (x + 1, y + 1), err_pixel, 1.0);
        }
    }
}

// TODO: FIXME, IM UGLY you fucking asshole
// this helper function will calculate the the neighbor pixel and add value from the error pixel as refrenced in wikipedia.
fn calculate_and_assign_pixel(
    img: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, // imagebuffer
    pixel_coord: (u32, u32),                               // coordinate (x,y)
    err_pixel: [f32; 3],                                   // error pixel [R, G, B]
    val: f32,                                              //value will be added to the calculation
) {
    // R
    img.get_pixel_mut(pixel_coord.0, pixel_coord.1).0[0] =
        (img.get_pixel(pixel_coord.0, pixel_coord.1).0[0] as f32 + err_pixel[0] * val / 16.0) as u8;
    // G
    img.get_pixel_mut(pixel_coord.0, pixel_coord.1).0[1] =
        (img.get_pixel(pixel_coord.0, pixel_coord.1).0[1] as f32 + err_pixel[1] * val / 16.0) as u8;
    // B
    img.get_pixel_mut(pixel_coord.0, pixel_coord.1).0[2] =
        (img.get_pixel(pixel_coord.0, pixel_coord.1).0[2] as f32 + err_pixel[2] * val / 16.0) as u8;
}

// this helper function to calculate the rgb values for the floyed dither algorithm.
fn find_closest_color(pixel: [u8; 3], factor: f32) -> [u8; 3] {
    [
        ((factor * pixel[0] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        ((factor * pixel[1] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        ((factor * pixel[2] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
    ]
}
