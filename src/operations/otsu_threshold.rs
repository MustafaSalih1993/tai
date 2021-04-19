use image::GrayImage;

pub trait OtsuThreshold {
    fn get_histogram(&mut self) -> [usize; 256];
    fn get_otsu_value(&mut self) -> u8;
    fn threshold(&mut self);
}

impl OtsuThreshold for GrayImage {
    fn get_histogram(&mut self) -> [usize; 256] {
        let mut out = [0; 256];
        self.iter().for_each(|p| {
            out[*p as usize] += 1;
        });
        out
    }
    fn get_otsu_value(&mut self) -> u8 {
        let img_histogram: [usize; 256] = self.get_histogram();
        let total_weight = self.width() as f64 * self.height() as f64;
        let mut bg_sum = 0.0;
        let mut bg_weight = 0.0;
        let mut max_variance = 0.0;
        let mut best_threshold = 0;
        let sum_intensity: f64 = img_histogram
            .iter()
            .enumerate()
            .fold(0f64, |acu, (t, c)| acu + (t * c) as f64);

        for (threshold, count) in img_histogram.iter().enumerate() {
            let fg_weight = total_weight - bg_weight;
            if fg_weight > 0.0 && bg_weight > 0.0 {
                let fg_mean = (sum_intensity - bg_sum) / fg_weight;
                let val = (bg_weight * fg_weight * ((bg_sum / bg_weight) - fg_mean)).powi(2);
                if val >= max_variance {
                    best_threshold = threshold as u8;
                    max_variance = val;
                }
            }
            bg_weight += *count as f64;
            bg_sum += (threshold * count) as f64;
        }

        best_threshold
    }
    fn threshold(&mut self) {
        let best_threshold = self.get_otsu_value();
        self.iter_mut()
            .for_each(|p| *p = if *p < best_threshold { 0 } else { 255 });
    }
}
