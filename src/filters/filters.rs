
use image::DynamicImage;

use crate::utils::configs::CONFIG;

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

