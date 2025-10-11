use std::thread;
use std::path::Path;
use std::time::Duration;

use image::{DynamicImage, RgbImage};
use video_rs::ffmpeg::decoder;
use video_rs::{Frame, Reader, Url};
use video_rs::decode::Decoder;
use ndarray::s;

use crate::img_filter::*;

pub fn get_video_decoder(path: &str) -> Decoder {
    let path = Path::new(path)
        .canonicalize()
        .expect(&format!("No such path {}", path));
    video_rs::init().unwrap();
    let url = Url::from_file_path(path)
        .expect("Failed to convert to url");
    
    Decoder::new(url).expect("Unable to open codec")
}

pub fn video_to_ascii(decoder: &mut Decoder){
    let sleep = 100; 
    for (i, frame_result) in decoder.decode_iter().enumerate() {
        println!("frame {}", i);
        if let Ok((_, frame)) = frame_result {
            let shape = frame.shape();
            let height = shape[0];
            image_to_ascii(scale_image(frame_to_dynamic_image(&frame), 100, 100));
            print!("\x1B[{}A", height);
        }
        thread::sleep(Duration::from_millis(sleep));
    }
}

pub fn frame_to_dynamic_image(frame: &ndarray::Array3<u8>) -> DynamicImage{
    let shape = frame.shape();
    let height = shape[0] as u32;
    let width = shape[1] as u32;

    let raw_pixels = frame.as_slice().expect("Frame slice failed");

    let img = RgbImage::from_raw(width, height, raw_pixels.to_vec())
        .expect("Failed to create RgbImage");

    DynamicImage::ImageRgb8(img)
}