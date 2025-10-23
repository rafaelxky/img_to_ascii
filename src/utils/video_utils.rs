use image::{DynamicImage, RgbImage};

#[allow(unused)]
pub fn frame_to_dynamic_image(frame: &ndarray::Array3<u8>) -> DynamicImage{
    let shape = frame.shape();
    let height = shape[0] as u32;
    let width = shape[1] as u32;

    let raw_pixels = frame.as_slice().expect("Frame slice failed");

    let img = RgbImage::from_raw(width, height, raw_pixels.to_vec())
        .expect("Failed to create RgbImage");

    DynamicImage::ImageRgb8(img)
}

#[allow(unused)]
pub fn move_cursor_to_top_image(din_image: &DynamicImage){
    print!("\x1B[{}A", din_image.height());
}

#[allow(unused)]
pub fn move_cursor_up(height: usize){
    print!("\x1B[{}A", height);
}