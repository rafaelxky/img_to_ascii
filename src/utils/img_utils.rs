use image::{imageops::FilterType, DynamicImage, Rgba, RgbaImage};
use wide::f32x8;

use crate::media::media_type::ResizeType;

#[allow(unused)]
pub fn scale_image( image: DynamicImage, width: u32, height: u32, resize_type: &ResizeType) -> DynamicImage {
    match resize_type {
        ResizeType::Fit => image.resize(width, height, FilterType::Lanczos3),
        ResizeType::Exact => image.resize_exact(width, height, FilterType::Lanczos3),
    }
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
pub fn simd_gray_image(image: &mut DynamicImage) {

    let img: &mut RgbaImage = match image.as_mut_rgba8() {
        Some(buf) => buf,
        None => {
            *image = DynamicImage::ImageRgba8(image.to_rgba8());
            image.as_mut_rgba8().unwrap()
        }
    };

    let pixels = img.as_mut(); 

    let len = pixels.len();
    const SIMD_CHUNK: usize = 8;

    let r_coeff = f32x8::splat(0.2126);
    let g_coeff = f32x8::splat(0.7152);
    let b_coeff = f32x8::splat(0.0722);

    let mut i = 0;
    while i + SIMD_CHUNK * 4 <= len {
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

    while i < len {
        let luma = (0.2126 * pixels[i] as f32
            + 0.7152 * pixels[i + 1] as f32
            + 0.0722 * pixels[i + 2] as f32) as u8;
        pixels[i] = luma;
        pixels[i + 1] = luma;
        pixels[i + 2] = luma;
        i += 4;
    }
}

