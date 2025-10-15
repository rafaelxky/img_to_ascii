use crate::utils::img_utils::{get_image, scale_image, simd_gray_image};
use crate::filters::ascii::image_to_ascii as static_image_to_ascii;
use crate::utils::video_utils::{get_video_decoder};
use crate::filters::marching_squares::image_to_marching_squares_ascii as msascii;
use crate::filters::marching_squares::video_to_marching_squares as vtms;
use crate::filters::ascii::video_to_ascii as raw_video_to_ascii;

#[allow(unused)]
pub fn video_to_ascii(path: &str, width: u32, height: u32, sleep: u64){
    raw_video_to_ascii(&mut get_video_decoder(path, width, height), width, height, sleep);
}

#[allow(unused)]
pub fn image_to_ascii(path: &str, width: u32, height: u32){
    static_image_to_ascii(&mut scale_image(get_image(path), width, height));
}

#[allow(unused)]
pub fn image_to_marching_squares(path: &String, width: u32, height: u32, layers: u8){
    msascii(&simd_gray_image(&mut scale_image(get_image(path), width, height)), layers);
}
#[allow(unused)]
pub fn video_to_marching_squares(path: &String, width: u32, height: u32, tolerance: u8, sleep: u64){
    vtms(&mut get_video_decoder(path, width, height), width, height,sleep, tolerance);
}