use std::path::Path;
use image::{DynamicImage, RgbImage};
use video_rs::{DecoderBuilder, Resize};
use video_rs::{Url};
use video_rs::decode::Decoder;


#[allow(unused)]
pub fn get_video_decoder(path: &str, width: u32, height: u32) -> Decoder {
    let path = Path::new(path)
        .canonicalize()
        .expect(&format!("No such path {}", path));
    video_rs::init().unwrap();

    let url = Url::from_file_path(path)
        .expect("Failed to convert to url");
    
     DecoderBuilder::new(url)
        .with_resize(Resize::Fit(width, height))
        .build().unwrap()
}


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