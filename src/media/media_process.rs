use std::thread;
use std::time::Duration;
use image::DynamicImage;
use video_rs::{decode::Decoder};
use crate::media::media_processor::{ApplyFilterChainFunc, FilterChainType, MediaOutputFunc};
use crate::utils::configs::FRAME_COUNTER;
use crate::utils::video_utils::{frame_to_dynamic_image};

/*
pub enum MediaProcessorType{
    ImageProcessor(fn(DynamicImage, ApplyFilterChainFunc, &Vec<fn(&mut DynamicImage)>, MediaOutputFunc)),
    VideoProcessor(fn(Decoder, u64, ApplyFilterChainFunc, &Vec<fn(&mut DynamicImage)>, MediaOutputFunc)),
}

pub type ApplyFilterChainFunc = fn(&Vec<fn(&mut DynamicImage)>, &mut DynamicImage);
pub type MediaOutputFunc =  fn(&mut DynamicImage);

*/

pub fn process_image(
    mut image: DynamicImage,
    apply_filter_chain: ApplyFilterChainFunc, 
    filter_chain: &FilterChainType, 
    output_media: MediaOutputFunc
){
    apply_filter_chain(filter_chain,&mut image);
    output_media(&mut image);
}

pub fn process_video(
    mut decoder: Decoder, 
    frame_delay: u64, 
    apply_filter_chain: ApplyFilterChainFunc, 
    filter_chain: &FilterChainType, 
    output_media: MediaOutputFunc
){
    loop {
        match decoder.decode() {
            Ok((_, frame)) => {
                let mut dimage = frame_to_dynamic_image(&frame);
                apply_filter_chain(filter_chain, &mut dimage);
                output_media(&mut dimage);
            }
            Err(video_rs::Error::DecodeExhausted) => {
                decoder.seek_to_start().unwrap();
            }
            Err(e) => {
                eprintln!("Decode error: {:?}", e);
                break;
            }
        }
        thread::sleep(Duration::from_millis(frame_delay));
        let mut frame_counter = FRAME_COUNTER.lock().unwrap();
        *frame_counter = *frame_counter + 1; 
    }
}