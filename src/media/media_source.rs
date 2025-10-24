use std::path::Path;
use image::{DynamicImage};
use video_rs::{DecoderBuilder, Resize};
use video_rs::{Url};
use video_rs::decode::Decoder;
use crate::media::media_type::ResizeType;
use crate::utils::img_utils::scale_image;

/*  
pub enum MediaSourceType {
    ImageSource(Box<dyn Fn(&str, u32, u32) -> DynamicImage>),
    VideoSource(Box<dyn Fn(&str, u32, u32) -> Decoder>),
}
*/

#[allow(unused)]
pub fn get_video_decoder(path: &str, width: u32, height: u32, resize_type: &ResizeType) -> Decoder {
    let path = Path::new(path)
        .canonicalize()
        .expect(&format!("No such path {}", path));
    video_rs::init().unwrap();

    let url = Url::from_file_path(path)
        .expect("Failed to convert to url");
    
    let mut decoder_builder = DecoderBuilder::new(url);

    decoder_builder = match resize_type {
        ResizeType::Exact => decoder_builder.with_resize(Resize::Exact(width, height)),
        ResizeType::Fit => decoder_builder.with_resize(Resize::Fit(width, height)),
    };

    decoder_builder.build().unwrap()
}

#[allow(unused)]
pub fn get_image(path: &str, width: u32, height: u32, resize_type: &ResizeType) -> DynamicImage {
    scale_image(image::open(path).expect("Error, could not get image!"), width, height, resize_type)
} 