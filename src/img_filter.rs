use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgba, Rgb};


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

pub fn image_to_ascii(image: DynamicImage, chars: &Vec<String>) {
    let step: usize = 255 / chars.len();
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = image.to_rgba8();
    for (_y, row) in img.rows().enumerate() {
        for (_x, pixel) in row.enumerate() {
            let mut index = (pixel_to_gray(pixel) as usize) / step;
            if index >= chars.len() {
                index = chars.len() - 1
            }
            let mut chara: &str = &chars[index];
            if pixel[3] == 0 {
                chara = " ";
            }
            print!("{}", chara);
        }
        println!();
    }
}