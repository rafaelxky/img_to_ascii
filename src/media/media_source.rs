use reqwest::blocking::{Client};
use std::io::{self, Read};
use std::io::Cursor;
use std::path::Path;
use image::{DynamicImage, ImageReader};
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
    scale_image(image::open(path).expect("Error, could not get image!"), width, height,resize_type)
} 

pub fn get_online_image(path: &str, width: u32, height: u32, resize_type: &ResizeType) -> DynamicImage {
    let client = Client::new();
    let resp = client.get(path).send().unwrap();
    let bytes = resp.bytes().unwrap();
    scale_image(
        ImageReader::new(Cursor::new(bytes))
    .with_guessed_format().unwrap()
    .decode().unwrap()
    , width, height, resize_type)
}

pub fn get_online_video(url: &str, width: u32, height: u32, resize_type: &ResizeType) -> Decoder {
     let mut decoder_builder = DecoderBuilder::new(Url::parse(url).unwrap());
    decoder_builder = match resize_type {
        ResizeType::Exact => decoder_builder.with_resize(Resize::Exact(width, height)),
        ResizeType::Fit => decoder_builder.with_resize(Resize::Fit(width, height)),
    };
    decoder_builder.build().unwrap()
}

pub fn get_image_from_bytes(width: u32, height: u32, resize_type: &ResizeType) -> DynamicImage{
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).expect("failed to read stdin");
    let cursor = Cursor::new(buf);
    let img = ImageReader::new(cursor)
        .with_guessed_format().unwrap()
        .decode().unwrap();
    return scale_image(img, width, height, resize_type);
}