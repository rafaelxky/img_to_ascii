use std::thread;
use std::path::Path;
use std::time::Duration;

use image::{DynamicImage, ImageBuffer, RgbImage, Rgba};
use video_rs::{DecoderBuilder, Resize};
use video_rs::{Url};
use video_rs::decode::Decoder;

use crate::img_filter::*;

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
pub fn video_to_ascii(decoder: &mut Decoder, width: u32, height: u32, sleep_millis: u64) {
    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    loop {
    match decoder.decode() {
        Ok((_, frame)) => {
            image_to_ascii(&frame_to_dynamic_image(&frame), &mut buffer);
            print!("\x1B[{}A", height);
        }
        Err(video_rs::Error::DecodeExhausted) => {
            decoder.seek_to_start().unwrap();
        }
        Err(e) => {
            eprintln!("Decode error: {:?}", e);
            break;
        }
    }

    thread::sleep(Duration::from_millis(sleep_millis));
    }

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
