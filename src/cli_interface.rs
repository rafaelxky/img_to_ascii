use std::{fs::{self}, path::Path, process::exit};

use infer;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use crate::{media::media_output::{ascii_output, colored_ascii_output, marching_squares_ascii_output}, utils::configs::ARGS};
use crate::media::media_processor::MediaProcessor;
use crate::media::media_process::{process_image,process_video};
use crate::media::media_source::{get_image, get_video_decoder};


#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum OutputOptions{
    ASCII,
    MSquares,
    CAscii,
}

#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
pub enum MediaType{
    Unknown,
    Unsuported,
    Video,
    Image,
}

// todo: implement
#[derive(Debug, Clone, ValueEnum, Serialize, Deserialize)]
#[allow(unused)]
pub enum LocationType{
    Local,
    Web,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(version, about, long_about = None)]
pub struct Args{

    /// Media source path
    pub file_path: String,
    /// Media resize width (defaut: terminal size)
    pub width: Option<u32>,
    /// Media resize height (default: terminal size)
    pub height: Option<u32>,
    /// Delay between frames in video (default: 50)
    #[arg(short = 'd', long)]
    pub frame_delay: Option<u64>,
    // filters
    /// How to output the media
    #[arg(short = 'o', long)]
    pub output_option: Option<OutputOptions>,

    /// Override custom configurations, can be donne manually in the config.json
    #[arg(short = 's', long)]
    pub set: Vec<String>,
}

pub fn handle_args() {
    let mut mp = MediaProcessor::new(ARGS.file_path.clone());
    let path = mp.get_path();

    if !file_exists(path) {
        println!("Error: file {} not found", path);
        std::process::exit(1);
    }

    // sources
    // processors
    match get_file_type(mp.get_path()) {
        MediaType::Video => { 
            mp.with_video_source(get_video_decoder);
            mp.with_process_video(process_video);
        },
        MediaType::Image => { 
            mp.with_image_source(get_image);
            mp.with_process_image(process_image);
        },
        MediaType::Unknown => { 
            println!("Error: filetype unknown!");
            exit(1);
        },
        MediaType::Unsuported => { 
            println!("Error, unsuported filetype!");
            exit(1);
        },
    }

    // output
    match &ARGS.output_option {
        Some(filter) => {
            match filter {
                OutputOptions::ASCII => {
                    mp.with_output(ascii_output);
                },
                OutputOptions::CAscii => {
                    mp.with_output(colored_ascii_output);
                },
                OutputOptions::MSquares => {
                    mp.with_output(marching_squares_ascii_output);
                }
            }
        }, 
        None => {
            mp.with_output(ascii_output);
        }
    }

    // frame delay
    if let Some(frame_delay) = ARGS.frame_delay {
        mp.with_frame_delay(frame_delay);
    }
    // size
    if let Some(width) = ARGS.width {
        mp.with_width(width);   
    }
    if let Some(height) = ARGS.height {
        mp.with_height(height);
    }

    mp.execute();
}

fn get_file_type(file_path: &str) -> MediaType {
    let data = fs::read(file_path).unwrap();
    if let Some(kind) = infer::get(&data) {
        if kind.mime_type().starts_with("image/") {
            return MediaType::Image;
        }
        if kind.mime_type().starts_with("video/") {
            return MediaType::Video;
        }
        return MediaType::Unsuported;
    }
    return MediaType::Unknown;
}

fn file_exists(path: &str) -> bool{
    Path::new(path).exists()
}