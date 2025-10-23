use video_rs::decode::Decoder;
use image::DynamicImage;

use crate::media_processor::MediaProcessor;

pub enum MediaType {
    Video(Decoder),
    Image(DynamicImage),
}
pub enum MediaProcessorType{
    ImageProcessor(fn(DynamicImage, fn(&Vec<fn(&mut DynamicImage)>, &mut DynamicImage), &Vec<fn(&mut DynamicImage)>)),
    VideoProcessor(fn(Decoder, u64, fn(&Vec<fn(&mut DynamicImage)>, &mut DynamicImage), &Vec<fn(&mut DynamicImage)>)),
}

pub enum MediaSourceType {
    ImageSource(Box<dyn Fn(&str, u32, u32) -> DynamicImage>),
    VideoSource(Box<dyn Fn(&str, u32, u32) -> Decoder>),
}