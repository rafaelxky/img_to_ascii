use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer};
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, Rgba, RgbaImage, Frame};
use std::{thread, time::Duration, fs::File, io::BufReader, io::{stdout, Write}};
use crossterm::terminal::{size};
mod img_filter;
mod video;
use crate::img_filter::{get_image, scale_image, image_to_ascii as static_image_to_ascii};
use crate::lookup_table::LOOKUP;
mod lookup_table;
use crate::video::{get_video_decoder, video_to_ascii as raw_video_to_ascii};

fn main() {
    // gradient from more to less bright
    let (term_width, term_height) = size().unwrap(); 
    let gradient: Vec<String> = LOOKUP.0.to_vec();

    //let path = "video.mp4";
    let path = "miku_dance.gif";
    
    //mp4, gif
    video_to_ascii(path, 100, 100, 50);

    // image
    //image_to_ascii(&gradient, "photo2.png", 100, 100);
}

#[allow(unused)]
fn video_to_ascii(path: &str, width: u32, height: u32, sleep: u64){
    raw_video_to_ascii(&mut get_video_decoder(path, width, height), width, height, sleep);
}

#[allow(unused)]
fn image_to_ascii(gradient: &Vec<String>, path: &str, width: u32, height: u32){
    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    static_image_to_ascii(&mut scale_image(get_image(path), width, height), &mut buffer);
}

#[allow(unused)]
fn print_gradient(gradient: &Vec<String>) {
    for _ in 0..5 {
        for char in gradient {
            print!("{}{}", char, char);
        }
        println!();
    }
}

#[allow(unused)]
fn animate_gradient(gradient: &Vec<String>) {
    let height = 5;
    let frames = 40;
    let sleep = 100;
    let mut gradient = gradient.clone();
    let reversed: Vec<String> = gradient.iter().rev().cloned().collect();
    gradient.extend(reversed);

    // frames
    for f in 0..frames {
        let mut buffer = String::new();
        // the println will make it go down automatically
        if f > 0 {
            buffer.push_str(&format!("\x1B[{}A", height));
        }
        // rows
        for _ in 0..height {
            for i in 0..gradient.len() {
                // i + f % gradient.len()
                let chara = &gradient[(f + i) % gradient.len()];
                buffer.push_str(&format!("{}{}", chara, chara));
            }
            buffer.push_str("\n");
        }
        print!("{}", buffer);
        thread::sleep(Duration::from_millis(sleep));
    }
}
