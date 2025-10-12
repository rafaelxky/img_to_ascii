use std::thread;
use std::path::Path;
use std::time::Duration;

use image::{DynamicImage, RgbImage};
use video_rs::ffmpeg::decoder;
use video_rs::{Frame, Reader, Url};
use video_rs::decode::Decoder;
use ndarray::s;

use crate::img_filter::*;
use std::sync::mpsc::channel;

pub fn get_video_decoder(path: &str) -> Decoder {
    let path = Path::new(path)
        .canonicalize()
        .expect(&format!("No such path {}", path));
    video_rs::init().unwrap();
    let url = Url::from_file_path(path)
        .expect("Failed to convert to url");
    
    Decoder::new(url).expect("Unable to open codec")
}

pub fn video_to_ascii(decoder: &mut Decoder, width: u32, height: u32, sleep_milis: u64){
    for (i, frame_result) in decoder.decode_iter().enumerate() {
        println!("Frame {}", i);
        if let Ok((_, frame)) = frame_result {
            image_to_ascii(scale_image(frame_to_dynamic_image(&frame), width, height));
            print!("\x1B[{}A", height + 1);
        }
        thread::sleep(Duration::from_millis(sleep_milis));
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
