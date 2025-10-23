use video_rs::decode::Decoder;
use image::DynamicImage;

use crate::media::media_processor::{ApplyFilterChainFunc, FilterChainType, MediaOutputFunc};

pub type ImageSourceFunc = fn(&str, u32, u32) -> DynamicImage;
pub type VideoSourceFunc = fn(&str, u32, u32) -> Decoder;
pub type ImageProcessorFunc = fn(DynamicImage, ApplyFilterChainFunc, &FilterChainType, MediaOutputFunc);
pub type VideoProcessorFunc = fn(Decoder, u64, ApplyFilterChainFunc, &FilterChainType, MediaOutputFunc);

pub enum MediaProcessorType{
    ImageProcessor(ImageProcessorFunc),
    VideoProcessor(VideoProcessorFunc),
}

pub enum MediaSourceType {
    ImageSource(ImageSourceFunc),
    VideoSource(VideoSourceFunc),
}