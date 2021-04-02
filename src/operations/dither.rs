use image::RgbaImage;
// This is error diff algorithm check the source below.
// source : https://en.wikipedia.org/wiki/Floyd-Steinberg_dithering

pub trait Dither {
    fn dither(&mut self, scale: u8);
    fn calculate_pixel(&mut self, pixel_coord: (u32, u32), err_pixel: [f32; 3], cal: f32);
}

impl Dither for RgbaImage {
    fn dither(&mut self, dither_scale: u8) {
        let scale = dither_scale as f32;

        for y in 0..self.height() - 1 {
            for x in 1..self.width() - 1 {
                let old_rgb: [u8; 4] = self.get_pixel(x, y).0;
                let new_rgb: [u8; 4] = find_closest_color(old_rgb, scale);

                self.get_pixel_mut(x, y).0[..3].clone_from_slice(&new_rgb[..3]);

                let mut pixel = self.get_pixel_mut(x, y).0;
                pixel[0] = new_rgb[0];
                pixel[1] = new_rgb[1];
                pixel[2] = new_rgb[2];

                let err_r: f32 = old_rgb[0] as f32 - new_rgb[0] as f32;
                let err_g: f32 = old_rgb[1] as f32 - new_rgb[1] as f32;
                let err_b: f32 = old_rgb[2] as f32 - new_rgb[2] as f32;
                let err_pixel = [err_r, err_g, err_b];

                self.calculate_pixel((x + 1, y), err_pixel, 7.0);
                self.calculate_pixel((x - 1, y + 1), err_pixel, 3.0);
                self.calculate_pixel((x, y + 1), err_pixel, 5.0);
                self.calculate_pixel((x + 1, y + 1), err_pixel, 1.0);
            }
        }
    }

    // this helper function will calculate the the neighbor pixel and add value from the error pixel as refrenced in wikipedia.
    fn calculate_pixel(
        &mut self,
        origin_pixel: (u32, u32), // coordinate (x,y)
        err_pixel: [f32; 3],      // error pixel [R, G, B]
        val: f32,                 // value will be added to the calculation
    ) {
        // R
        self.get_pixel_mut(origin_pixel.0, origin_pixel.1).0[0] = (self
            .get_pixel(origin_pixel.0, origin_pixel.1)
            .0[0] as f32
            + err_pixel[0] * val / 16.0)
            as u8;
        // G
        self.get_pixel_mut(origin_pixel.0, origin_pixel.1).0[1] = (self
            .get_pixel(origin_pixel.0, origin_pixel.1)
            .0[1] as f32
            + err_pixel[1] * val / 16.0)
            as u8;
        // B
        self.get_pixel_mut(origin_pixel.0, origin_pixel.1).0[2] = (self
            .get_pixel(origin_pixel.0, origin_pixel.1)
            .0[2] as f32
            + err_pixel[2] * val / 16.0)
            as u8;
    }
}

// this helper function to calculate the rgb values for the floyed dither algorithm.
fn find_closest_color(pixel: [u8; 4], factor: f32) -> [u8; 4] {
    [
        ((factor * pixel[0] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        ((factor * pixel[1] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        ((factor * pixel[2] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        pixel[3],
    ]
}
