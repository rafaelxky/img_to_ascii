use video_rs::decode::Decoder;
use image::DynamicImage;

use crate::media_processor::{ApplyFilterChainFunc, MediaOutputFunc};

pub enum MediaType {
    Video(Decoder),
    Image(DynamicImage),
}
pub enum MediaProcessorType{
    ImageProcessor(fn(DynamicImage, ApplyFilterChainFunc, &Vec<fn(&mut DynamicImage)>, MediaOutputFunc)),
    VideoProcessor(fn(Decoder, u64, ApplyFilterChainFunc, &Vec<fn(&mut DynamicImage)>, MediaOutputFunc)),
}



pub enum MediaSourceType {
    ImageSource(Box<dyn Fn(&str, u32, u32) -> DynamicImage>),
    VideoSource(Box<dyn Fn(&str, u32, u32) -> Decoder>),
}