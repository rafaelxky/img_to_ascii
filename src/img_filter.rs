use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgba, Rgb};
use std::{io::{stdout, Write}};

use crate::lookup_table::LOOKUP;


pub fn get_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
} 

pub fn scale_image( image: DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, FilterType::Lanczos3)
}

pub fn gray_image(image: &mut DynamicImage) {
    let mut rgb = image.to_rgba8();

    for (_, _, pixel) in rgb.enumerate_pixels_mut() {
        let luma = pixel_to_gray(pixel);
        *pixel = Rgba([luma, luma, luma, pixel[3]]);
    }

    *image = DynamicImage::ImageRgba8(rgb);
}

pub fn save_image(image: &mut DynamicImage, name: &str){
    image.save(name);
}

pub fn pixel_to_gray(pixel: &Rgba<u8>) -> u8 {
            (0.2126 * pixel[0] as f32
            + 0.7152 * pixel[1] as f32
            + 0.0722 * pixel[2] as f32) as u8
}

pub fn image_to_ascii(image: DynamicImage) {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image.to_rgba8();
    let mut buffer = String::with_capacity((img.width() * img.height()) as usize);
    for (_y, row) in img.rows().enumerate() {
        for (_x, pixel) in row.enumerate() {
            let gray = pixel_to_gray(pixel);
            let mut chara: &str = &LOOKUP[gray as usize];
            if pixel[3] == 0 {
                chara = " ";
            }
            buffer.push_str(chara);
        }
        buffer.push_str("\n");
    }
    print!("{}", buffer);
    stdout().flush().unwrap(); 
}

