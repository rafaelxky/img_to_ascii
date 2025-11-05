use std::{process::exit};
use crossterm::terminal::size;
use image::DynamicImage;

use crate::media::media_output::ascii_output;
use crate::{utils::configs::CONFIG};
use crate::media::{media_type::*};

pub type ApplyFilterChainFunc = fn(&Vec<fn(&mut DynamicImage)>, &mut DynamicImage);
pub type MediaOutputFunc =  fn(&mut DynamicImage);
pub type FilterFunc = fn(&mut DynamicImage);
pub type FilterChainType =  Vec<FilterFunc>;

pub struct MediaProcessor {
    pub file_path: String,
    pub width: u32,
    pub height: u32,
    pub frame_delay: u64,
    pub resize_type: ResizeType,
    // were the media comes from
    pub source_media: Option<MediaSourceType>,
    // applies filter chain and controls the other functions execution
    pub process_media: Option<MediaProcessorType>,
    // the list of filters to apply
    // todo: implement
    pub filter_chain: FilterChainType,
    // how to output the media
    pub media_output: Option<MediaOutputFunc>,
}

impl MediaProcessor {

  pub fn new(file_path: String) -> Self {
    let (mut width, mut height) = (100, 100);

    if let Ok((width_result, height_result)) = size() {
        width = width_result;
        height = height_result;
    } else {
        #[cfg(debug_assertions)]
        println!("Error: could not determine terminal size!");
    }

    let mut frame_delay = 50;

    if let Ok(config) = CONFIG.read() {
        frame_delay = config.default_frame_delay;
    } else {
        #[cfg(debug_assertions)]
        println!("Error: could not obtain read lock on config in MediaProcessor initialisation!");
    }

        Self {
            file_path,
            width: (width - 5) as u32, 
            height: (height - 5) as u32,
            frame_delay: frame_delay,
            source_media: None,
            process_media: None,
            filter_chain: Vec::new(),
            media_output: Some(ascii_output),
            resize_type: ResizeType::Fit,
        }
    }


    //path
    pub fn get_path(&self) -> &str {
        &self.file_path
    }

    pub fn with_resize_type(&mut self, resize_type: ResizeType) -> &mut Self{
        self.resize_type = resize_type;
        self
    }

    //sizes
    pub fn with_width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }
    pub fn with_height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }
    // delay
    pub fn with_frame_delay(&mut self, frame_delay: u64) -> &mut Self {
        self.frame_delay = frame_delay;
        self
    }
    // sources 
    pub fn with_image_source(&mut self, image_provider: ImageSourceFunc ) -> &mut Self{
        self.source_media = Some(MediaSourceType::ImageSource(image_provider));
        self
    }
    pub fn with_image_byte_source(&mut self, image_byte_provider: ByteImageSourceFunc) -> &mut Self{
        self.source_media = Some(MediaSourceType::ByteImageSource(image_byte_provider));
        self
    }
    pub fn with_video_source(&mut self, video_decoder_provider: VideoSourceFunc) -> &mut Self {
        self.source_media = Some(MediaSourceType::VideoSource(video_decoder_provider));
        self
    }
    // processors
    pub fn with_process_image(&mut self, image_processor: ImageProcessorFunc) -> &mut Self {
        self.process_media = Some(MediaProcessorType::ImageProcessor(image_processor));
        self
    }

    pub fn with_process_video(&mut self, video_processor: VideoProcessorFunc) -> &mut Self {
        self.process_media = Some(MediaProcessorType::VideoProcessor(video_processor));
        self
    }

    // filter chain
    #[allow(unused)]
    pub fn add_filter(&mut self, filter: FilterFunc) -> &mut Self{
        self.filter_chain.push(filter);
        self
    }

    // output 
    pub fn with_output(&mut self, media_output: MediaOutputFunc) -> &mut Self {
        self.media_output = Some(media_output);
        self
    }

    pub fn execute(&mut self){
        // if guards for none
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
           if let MediaProcessorType::ImageProcessor(process_image) = media_processor{

                let image = match media_source {
                    MediaSourceType::ImageSource(source_image) => {
                        source_image(&self.file_path, self.width, self.height, &self.resize_type)
                    }
                    MediaSourceType::ByteImageSource(source_image) => {
                        source_image(self.width, self.height, &self.resize_type)
                    }
                    _ => {
                        eprintln!("Error: media processor and source mismatch!");
                        std::process::exit(1);
                    }
                };
                process_image(image, apply_filter_chain, &self.filter_chain, media_output);
           } else  
           // video
           if let (MediaProcessorType::VideoProcessor(process_video), MediaSourceType::VideoSource(source_video)) = (media_processor, media_source){
                process_video(source_video(&self.file_path, self.width, self.height, &self.resize_type), self.frame_delay, apply_filter_chain, &self.filter_chain, media_output);
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