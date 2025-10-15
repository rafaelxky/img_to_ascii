use crate::utils::{img_utils::*};
use image::{DynamicImage};
use crate::utils::configs::{LOOKUP};

pub fn image_to_ascii(image: &mut DynamicImage) {

    let gray_image = simd_gray_image(image).to_rgba8();
    let width = gray_image.width() as usize;
    let height = gray_image.height() as usize;
    let mut ascii = String::with_capacity(width * height + height);

    for y in 0..height {
        for x in 0..width {
            let pixel = gray_image.get_pixel(x as u32, y as u32);
            let gray = pixel[0];
            let c = if pixel[3] == 0 { " " } else { &LOOKUP.0[gray as usize] };
            ascii.push_str(c);
        }
        ascii.push('\n');
    }

    print!("{}", ascii);
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}
