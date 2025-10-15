use std::{io::BufWriter};
use image::{DynamicImage, GenericImageView};
use crate::utils::{configs::LOOKUP};
use crate::utils::img_utils::pixel_to_gray;
use std::io::Write;


pub fn image_to_colored_ascii (image: &mut DynamicImage) {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut buffer = BufWriter::new(std::io::stdout());

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel( x as u32, y as u32);
            if pixel[3] == 0 {
                write!(buffer, " ").unwrap();
                continue;
            }
            let gray = pixel_to_gray(&pixel);
            let c = if gray == 0 { " " } else { &LOOKUP.0[gray as usize] };
            write!(buffer, "\x1b[38;2;{};{};{}m{}\x1b[0m",pixel[0],pixel[1],pixel[2],c).unwrap();
        }
        writeln!(buffer).unwrap();
    }

    buffer.flush().unwrap();
} 