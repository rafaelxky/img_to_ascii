use std::{process::exit};

use crossterm::terminal::size;
use image::DynamicImage;
use video_rs::decode::Decoder;


use crate::{media_processor, media_type::{MediaProcessorType, MediaSourceType, MediaType}, utils::configs::CONFIG};

pub struct MediaProcessor<'a> {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub frame_delay: u64,
    pub source_media: Option<MediaSourceType>,
    pub process_media: Option<MediaProcessorType>,
    pub filter_chain: Vec<Box<dyn Fn(&mut DynamicImage) -> DynamicImage + 'a>>,
    pub media_output: Option<Box<dyn Fn(&mut DynamicImage) + 'a>>,
    //pub get_image: Option<Box<dyn Fn(&str, u32, u32) -> DynamicImage + 'a>>,
    //pub get_video_decoder: Option<Box<dyn Fn(&str, u32, u32) -> Decoder + 'a>>,
    //pub process_image: Option<Box<dyn Fn(&mut DynamicImage) + 'a>>,
    //pub process_video: Option<Box<dyn Fn(&mut Decoder, u64, &Box<dyn Fn(&mut DynamicImage) + 'a>) + 'a>>,
}

impl <'a> MediaProcessor<'a> {

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
            //get_image: None,
            //get_video_decoder: None,
            //process_image: None,
            //process_video: None,
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
    pub fn with_process_image(&mut self, image_processor: &'static dyn Fn(&mut DynamicImage)) -> &mut Self {
        self.process_media = Some(MediaProcessorType::ImageProcessor(Box::from(image_processor)));
        self
    }
    pub fn with_process_video(&mut self, video_processor: &'static dyn Fn(&mut Decoder, u64, &Box<dyn Fn(&mut DynamicImage) + 'a>)) -> &mut Self {
        self.process_media = Some(MediaProcessorType::VideoProcessor(Box::from(video_processor)));
        self
    }
    pub fn with_get_image(&mut self, image_provider: &'static dyn Fn(&str, u32, u32) -> DynamicImage ) -> &mut Self{
        self.source_media = Some(MediaSourceType::ImageSource(Box::from(image_provider)));
        self
    }
    pub fn with_get_video_decoder(&mut self, video_decoder_provider: &'static dyn Fn(&str, u32, u32) -> Decoder) -> &mut Self {
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

       if let (Some(media_processor), Some(media_source), Some(media_output)) = (&self.process_media, &self.source_media, &self.media_output){
            // image
           if let (MediaProcessorType::ImageProcessor(process_image), MediaSourceType::ImageSource(source_image)) = (media_processor, media_source){
                process_image(source_image(&self.file_path, self.width, self.height), Box::new(|_,frame| self.apply_filter_chain(frame)));
           } else  
           // video
           if let (MediaProcessorType::VideoProcessor(process_video), MediaSourceType::VideoSource(source_video)) = (media_processor, media_source){
                process_video(source_video(&self.file_path, self.width, self.height), self.frame_delay, Box::new(|_,img| self.apply_filter_chain(img)));
           } else {
                println!("Error, media processor and media provider missmatch!");
                exit(1);
           }
       } 
    }

    pub fn apply_filter_chain(&self, image: &mut  DynamicImage) {
        self.filter_chain.iter().for_each(|filter| {
            filter(image);
        });
    }
}
