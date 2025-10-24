
use image::{DynamicImage, GenericImage, GenericImageView};

use crate::utils::configs::{CONFIG, FRAME_COUNTER};

pub fn rotate90(image: &mut DynamicImage){
    *image = image.rotate90();
}

pub fn rotate180(image: &mut DynamicImage){
    *image = image.rotate180();
}

pub fn blur(image: &mut DynamicImage){
    *image = image.blur(CONFIG.blur_sigma);
}

pub fn gray(image: &mut DynamicImage){
    *image = image.grayscale();
}

pub fn invert_color(image: &mut DynamicImage){
    image.invert();
}

pub fn wave(image: &mut DynamicImage) {
    let mut image_buffer = DynamicImage::new_rgba8(image.width(), image.height());
    let counter = FRAME_COUNTER.lock().unwrap();
    let width = image.width() as i32;
    let height = image.height();

    for y in 0..height {
        let wave_amplitude: f32 = CONFIG.wave_amplitude;  
        let wave_frequency: f32 = CONFIG.wave_frequency;   
        let offset = ((y as f32 * wave_frequency + *counter as f32 * 0.1).sin() * wave_amplitude) as i32;

        for x in 0..width {
            let target_x = (x + offset).rem_euclid(width); 
            let pixel = image.get_pixel(x as u32, y);
            image_buffer.put_pixel(target_x as u32, y, pixel);
        }
    }

    *image = image_buffer;
}

