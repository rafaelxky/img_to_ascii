use video_rs::decode::Decoder;
use image::DynamicImage;

use crate::media_processor::MediaProcessor;

pub enum MediaType {
    Video(Decoder),
    Image(DynamicImage),
}

pub enum MediaProcessorType<'b> {
    ImageProcessor(Box<dyn Fn(DynamicImage, Box<dyn Fn(&'b MediaProcessor<'b>, &mut DynamicImage)>)>),
    VideoProcessor(Box<dyn Fn(Decoder, u64, Box<dyn Fn(&'b MediaProcessor<'b>, &mut DynamicImage)>)>),
}

pub enum MediaSourceType {
    ImageSource(Box<dyn Fn(&str, u32, u32) -> DynamicImage>),
    VideoSource(Box<dyn Fn(&str, u32, u32) -> Decoder>),
}