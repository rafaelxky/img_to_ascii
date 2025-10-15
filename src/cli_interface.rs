use std::{fs::{self, exists}, process::exit};

use crate::{facade::*};
use crossterm::terminal::{size};
use infer;
use clap::{Parser, ValueEnum};
use video_rs::ffmpeg::filter;

#[derive(Debug, Clone, ValueEnum)]
enum FilterOptions{
    ASCII,
    MarchingSquares,
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

    #[arg(short = 't', long)]
    tolerance: Option<u8>
}

pub fn handle_args() {
    let args = Args::parse();
    let (term_width, term_height) = size().unwrap(); 

    let width: u32 = match args.width {
        Some(width) => { width },
        None => { (term_width - 20) as u32}
    };
    let height: u32 = match args.height {
        Some(height) => { height },
        None => { (term_height - 20) as u32 }
    };

    let fp = args.file_path.to_lowercase();
    if !exists(&fp).expect("Error checking file existance!") {
        println!("{} is not a valid file", &fp);
        exit(1);
    }
    let data = fs::read(fp).unwrap();
    if let Some(kind) = infer::get(&data) {
        if kind.mime_type().starts_with("image/") {
            match args.filter_option {
                Some( filter ) => { match filter {
                    FilterOptions::ASCII => {
                        image_to_ascii(&args.file_path, width, height);
                    },
                    FilterOptions::MarchingSquares => {
                        image_to_marching_squares(&args.file_path, width, height, args.tolerance.unwrap_or(50));
                    }
                }},
                None => { 
                    image_to_ascii(&args.file_path, width, height);
                }
            }
        } else if kind.mime_type().starts_with("video/") {
            match args.filter_option {
                Some(filter) => {
                    match filter {
                        FilterOptions::ASCII => {
                            video_to_ascii(&args.file_path, width, height, args.frame_delay.unwrap_or(50));
                        },
                        FilterOptions::MarchingSquares => {
                            video_to_marching_squares(&args.file_path, width, height, args.tolerance.unwrap_or(2), args.frame_delay.unwrap_or(50));
                        }
                    }
                },
                None => {
                    video_to_ascii(&args.file_path, width, height, args.frame_delay.unwrap_or(50));
                }
            }
        } else {
            println!("Unsuported file type!");
            exit(1);
        }
    } else {
        println!("Unknown file type!");
        exit(1);
    }
}

