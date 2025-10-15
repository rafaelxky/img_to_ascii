use std::{fs::{self}, path::Path, process::exit};

use crate::{filters::{ascii::image_to_ascii, colored_ascii::image_to_colored_ascii, marching_squares::image_to_marching_squares_ascii}, media_processor::MediaProcessor, utils::{img_utils::get_image, video_utils::{get_video_decoder, process_frames}}};
use infer;
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
enum FilterOptions{
    ASCII,
    MSquares,
    CAscii,
}

enum FileType{
    Unknown,
    Unsuported,
    Video,
    Image,
}

#[allow(unused)]
enum LocationType{
    Local,
    Web,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{

    file_path: String,
    width: Option<u32>,
    height: Option<u32>,

    #[arg(short = 'd', long)]
    frame_delay: Option<u64>,
    
    #[arg(short = 'f', long)]
    filter_option: Option<FilterOptions>,
}

pub fn handle_args() {
    let args = Args::parse();
    let mut mp = MediaProcessor::new(args.file_path);
    let path = mp.get_path();

    if !file_exists(path) {
        println!("Error: file {} not found", path);
        std::process::exit(1);
    }

    match get_file_type(mp.get_path()) {
        FileType::Video => { 
            mp.with_get_video_decoder(&get_video_decoder);
            mp.with_process_video(&process_frames);
        },
        FileType::Image => { 
            mp.with_get_image(&get_image);
        },
        FileType::Unknown => { 
            println!("Error: filetype unknown!");
            exit(1);
        },
        FileType::Unsuported => { 
            println!("Error, unsuported filetype!");
            exit(1);
        },
    }

    match args.filter_option {
        Some(filter) => {
            match filter {
                FilterOptions::ASCII => {
                    mp.with_process_image(&image_to_ascii);
                },
                FilterOptions::CAscii => {
                    mp.with_process_image(&image_to_colored_ascii);
                },
                FilterOptions::MSquares => {
                    mp.with_process_image(&image_to_marching_squares_ascii);
                }
            }
        }, 
        None => {
            mp.with_process_image(&image_to_ascii);
        }
    }

    if let Some(frame_delay) = args.frame_delay {
        mp.with_frame_delay(frame_delay);
    }
    if let Some(width) = args.width {
        mp.with_width(width);   
    }
    if let Some(height) = args.height {
        mp.with_height(height);
    }

    mp.execute();
}

fn get_file_type(file_path: &str) -> FileType {
    let data = fs::read(file_path).unwrap();
    if let Some(kind) = infer::get(&data) {
        if kind.mime_type().starts_with("image/") {
            return FileType::Image;
        }
        if kind.mime_type().starts_with("video/") {
            return FileType::Video;
        }
        return FileType::Unsuported;
    }
    return FileType::Unknown;
}

fn file_exists(path: &str) -> bool{
    Path::new(path).exists()
}