use image::{ImageBuffer};
use image::{Rgba};
use crate::img_filter::{get_image, scale_image, image_to_ascii as static_image_to_ascii};
use crate::video::{get_video_decoder, video_to_ascii as raw_video_to_ascii};

#[allow(unused)]
pub fn video_to_ascii(path: &str, width: u32, height: u32, sleep: u64){
    raw_video_to_ascii(&mut get_video_decoder(path, width, height), width, height, sleep);
}

#[allow(unused)]
pub fn image_to_ascii(path: &str, width: u32, height: u32){
    static_image_to_ascii(&mut scale_image(get_image(path), width, height));
}