use std::{process::exit};

use crossterm::terminal::size;
use image::DynamicImage;
use video_rs::{decode::Decoder, ffmpeg::media};


use crate::{media_processor, media_type::{MediaProcessorType, MediaSourceType, MediaType}, utils::configs::CONFIG};

pub type ApplyFilterChainFunc = fn(&Vec<fn(&mut DynamicImage)>, &mut DynamicImage);
pub type MediaOutputFunc =  fn(&mut DynamicImage);

pub struct MediaProcessor {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub frame_delay: u64,
    // were the media comes from
    pub source_media: Option<MediaSourceType>,
    // wrapper around apply filter chain function to controll its execution
    pub process_media: Option<MediaProcessorType>,
    // the list of filters to apply
    pub filter_chain: Vec<fn(&mut DynamicImage)>,
    // how to output the media
    pub media_output: Option<MediaOutputFunc>,
}

impl MediaProcessor {

  pub fn new(file_path: String) -> Self {
    let (width, height) = size().unwrap();
        Self {
            file_path,
            width: (width - 5) as u32, 
            height: (height - 5) as u32,
            frame_delay: CONFIG.default_frame_delay,
            source_media: None,
            process_media: None,
            media_output: None,
            filter_chain: Vec::new(),
        }
    }

    pub fn get_path(&self) -> &str {
        &self.file_path
    }

    pub fn with_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }
    pub fn with_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
    pub fn with_frame_delay(&mut self, frame_delay: u64) -> &mut Self {
        self.frame_delay = frame_delay;
        self
    }
    pub fn with_process_image(&mut self, image_processor: fn(DynamicImage, ApplyFilterChainFunc, &Vec<fn(&mut DynamicImage)>, MediaOutputFunc)) -> &mut Self {
        self.process_media = Some(MediaProcessorType::ImageProcessor(image_processor));
        self
    }

    pub fn with_process_video(&mut self, video_processor: fn(Decoder, u64, ApplyFilterChainFunc, &Vec<fn(&mut DynamicImage)>, MediaOutputFunc)) -> &mut Self {
        self.process_media = Some(MediaProcessorType::VideoProcessor(video_processor));
        self
    }

    pub fn with_get_image(&mut self, image_provider: fn(&str, u32, u32) -> DynamicImage ) -> &mut Self{
        self.source_media = Some(MediaSourceType::ImageSource(Box::from(image_provider)));
        self
    }
    pub fn with_get_video_decoder(&mut self, video_decoder_provider: fn(&str, u32, u32) -> Decoder) -> &mut Self {
        self.source_media = Some(MediaSourceType::VideoSource(Box::from(video_decoder_provider)));
        self
    }

    pub fn execute(&mut self){
       if self.source_media.is_none() {
            println!("Error, no media source provided!");
            exit(1);
       }
       if self.process_media.is_none() {
            println!("Error, no media processor provided!");
            exit(1);
       }
       if self.media_output.is_none(){
            println!("Error, no media output provided!");
            exit(1);
       }

       if let (Some(media_processor), Some(media_source), Some(media_output)) = (&self.process_media, &self.source_media, self.media_output){
            // image
           if let (MediaProcessorType::ImageProcessor(process_image), MediaSourceType::ImageSource(source_image)) = (media_processor, media_source){
                process_image(source_image(&self.file_path, self.width, self.height), apply_filter_chain, &self.filter_chain, media_output);
           } else  
           // video
           if let (MediaProcessorType::VideoProcessor(process_video), MediaSourceType::VideoSource(source_video)) = (media_processor, media_source){
                process_video(source_video(&self.file_path, self.width, self.height), self.frame_delay, apply_filter_chain, &self.filter_chain, media_output);
           } else {
                println!("Error, media processor and media provider missmatch!");
                exit(1);
           }
       } 
    }

    
}

pub fn apply_filter_chain(filter_chain: &Vec<fn(&mut DynamicImage)>, image: &mut  DynamicImage) {
        filter_chain.iter().for_each(|filter| {
            filter(image);
        });
    }