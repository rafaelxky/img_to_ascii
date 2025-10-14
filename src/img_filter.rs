use image::{imageops::FilterType, DynamicImage, ImageBuffer, Rgba};

use crate::lookup_table::LOOKUP;
use wide::f32x8;


#[allow(unused)]
pub fn get_image(path: &str) -> DynamicImage {
    image::open(path).unwrap()
} 

#[allow(unused)]
pub fn scale_image( image: DynamicImage, width: u32, height: u32) -> DynamicImage {
    image.resize(width, height, FilterType::Lanczos3)
}

#[allow(unused)]
pub fn image_to_gray(image: &mut DynamicImage) {
    let mut rgb = image.to_rgba8();

    for (_, _, pixel) in rgb.enumerate_pixels_mut() {
        let luma = pixel_to_gray(pixel);
        *pixel = Rgba([luma, luma, luma, pixel[3]]);
    }

    *image = DynamicImage::ImageRgba8(rgb);
}

#[allow(unused)]
pub fn save_image(image: &mut DynamicImage, name: &str){
    image.save(name);
}

#[allow(unused)]
pub fn pixel_to_gray(pixel: &Rgba<u8>) -> u8 {
            (0.2126 * pixel[0] as f32
            + 0.7152 * pixel[1] as f32
            + 0.0722 * pixel[2] as f32) as u8
}

#[allow(unused)]
pub fn simd_gray_image(image: DynamicImage) -> DynamicImage {
    let mut img = image.to_rgba8();
    let pixels = img.as_mut(); // &mut [u8], flat RGBA bytes

    let len = pixels.len();
    const SIMD_CHUNK: usize = 8; // 8 pixels per SIMD batch

    let r_coeff = f32x8::splat(0.2126);
    let g_coeff = f32x8::splat(0.7152);
    let b_coeff = f32x8::splat(0.0722);

    let mut i = 0;
    while i + SIMD_CHUNK * 4 <= len {
        // Load 8 pixels (RGBA = 4 bytes per pixel)
        let mut r = [0f32; SIMD_CHUNK];
        let mut g = [0f32; SIMD_CHUNK];
        let mut b = [0f32; SIMD_CHUNK];

        for j in 0..SIMD_CHUNK {
            let idx = i + j * 4;
            r[j] = pixels[idx] as f32;
            g[j] = pixels[idx + 1] as f32;
            b[j] = pixels[idx + 2] as f32;
        }

        let r_v = f32x8::from(r);
        let g_v = f32x8::from(g);
        let b_v = f32x8::from(b);

        let gray = (r_v * r_coeff) + (g_v * g_coeff) + (b_v * b_coeff);

        let gray_arr: [f32; SIMD_CHUNK] = gray.into();

        for j in 0..SIMD_CHUNK {
            let idx = i + j * 4;
            let luma = gray_arr[j] as u8;
            pixels[idx] = luma;
            pixels[idx + 1] = luma;
            pixels[idx + 2] = luma;
        }

        i += SIMD_CHUNK * 4;
    }

    // process remaining pixels scalar way
    while i < len {
        let luma = (0.2126 * pixels[i] as f32
            + 0.7152 * pixels[i + 1] as f32
            + 0.0722 * pixels[i + 2] as f32) as u8;
        pixels[i] = luma;
        pixels[i + 1] = luma;
        pixels[i + 2] = luma;
        i += 4;
    }

    DynamicImage::ImageRgba8(img)
}

#[allow(unused)]
pub fn image_to_ascii(image: &DynamicImage, buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    image
        .to_rgba8()
        .enumerate_pixels()
        .for_each(|(x, y, px)| buffer.put_pixel(x, y, *px));

    let width = buffer.width() as usize;
    let height = buffer.height() as usize;
    let mut ascii = String::with_capacity(width * height + height);

    for y in 0..height {
        for x in 0..width {
            let pixel = buffer.get_pixel(x as u32, y as u32);
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

