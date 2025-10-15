use std::{process::exit};

use crossterm::terminal::size;
use image::DynamicImage;
use video_rs::decode::Decoder;

use crate::utils::configs::CONFIG;

pub struct MediaProcessor<'a> {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub frame_delay: u64,
    pub get_image: Option<Box<dyn Fn(&str, u32, u32) -> DynamicImage + 'a>>,
    pub get_video_decoder: Option<Box<dyn Fn(&str, u32, u32) -> Decoder + 'a>>,
    pub process_image: Option<Box<dyn Fn(&mut DynamicImage) + 'a>>,
    pub process_video: Option<Box<dyn Fn(&mut Decoder, u64, &Box<dyn Fn(&mut DynamicImage) + 'a>) + 'a>>,
}

impl <'a> MediaProcessor<'a> {

  pub fn new(file_path: String) -> Self {
    let (width, height) = size().unwrap();
        Self {
            file_path,
            width: (width - 5) as u32, 
            height: (height - 5) as u32,
            frame_delay: CONFIG.default_frame_delay,
            get_image: None,
            get_video_decoder: None,
            process_image: None,
            process_video: None,
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
    pub fn with_process_image(&mut self, image_processor: &'a dyn Fn(&mut DynamicImage)) -> &mut Self {
        self.process_image = Some(Box::from(image_processor));
        self
    }
    pub fn with_process_video(&mut self, video_processor: &'a dyn Fn(&mut Decoder, u64, &Box<dyn Fn(&mut DynamicImage) + 'a>)) -> &mut Self {
        self.process_video = Some(Box::from(video_processor));
        self
    }
    pub fn with_get_image(&mut self, image_provider: &'a dyn Fn(&str, u32, u32) -> DynamicImage ) -> &mut Self{
        self.get_image = Some(Box::from(image_provider));
        self
    }
    pub fn with_get_video_decoder(&mut self, video_decoder_provider: &'a dyn Fn(&str, u32, u32) -> Decoder) -> &mut Self {
        self.get_video_decoder = Some(Box::from(video_decoder_provider));
        self
    }

    pub fn execute(self){
        if let Some(ref process_image) = self.process_image {
            if let (Some(ref process_video), Some(ref get_video_decoder)) = (self.process_video, self.get_video_decoder) {
                process_video(&mut get_video_decoder(&self.file_path, self.width, self.height), self.frame_delay, process_image);
            } else 
            if let Some(ref get_image) = self.get_image {
                process_image(&mut get_image(&self.file_path, self.width, self.height));
            } else {
                println!("Error, no image provider provided!");
                exit(1);
            }
        } else {
            println!("Error, no image processor provided!");
            exit(1);
        }
    }
}