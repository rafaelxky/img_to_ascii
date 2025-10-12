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

fn main() {
    // gradient from more to less bright
    let (term_width, term_height) = size().unwrap(); 
    let gradient: Vec<String> = LOOKUP.0.to_vec();
    let char_count: usize = LOOKUP.1;

    let path = "miku_dance.gif";
    
    //video_to_ascii(&mut get_video_decoder(path), 50, 50, 1);
    //frames_to_ascii(gif_to_gray(resize_gif(get_gif_frames(path), 100,100)), &gradient);

    //gif
    gif_to_ascii(&gradient, char_count, path, 100, 100);
    // image
    //image_to_ascii(&gradient, "miku.png", 100, 100);
}

fn image_to_ascii(gradient: &Vec<String>, path: &str, width: u32, height: u32){
    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    static_image_to_ascii(&mut scale_image(get_image(path), width, height), &mut buffer);
}

fn gif_to_ascii(gradient: &Vec<String>, char_count: usize ,path: &str ,width: u32, height: u32){
    frames_to_ascii(gif_to_gray(resize_gif(get_gif_frames(path),width, height)), &gradient);
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


#[allow(unused)]
fn get_gif_frames(path: &str) -> Vec<RgbaImage> {
    let file = File::open(path).expect("Cannot open GIF");
    let reader = BufReader::new(file);

    let decoder = GifDecoder::new(reader).expect("Not a GIF");

    decoder.into_frames()
        // todo: here
        // collect frames loads frames into ram, this needs to be changed
        .collect_frames() 
        .expect("Error reading frames")
        .into_iter()
        .map(|frame: Frame| frame.into_buffer())
        .collect()
}

#[allow(unused)]
fn resize_gif(frames: Vec<RgbaImage>, new_width: u32, new_height: u32) -> Vec<RgbaImage> {
    frames.into_iter()
        .map(|frame| {
            let dyn_img = DynamicImage::ImageRgba8(frame);
            dyn_img.resize(new_width, new_height, FilterType::Lanczos3).to_rgba8()
        })
        .collect()
}

#[allow(unused)]
fn gif_to_gray(frames: Vec<RgbaImage>) -> Vec<RgbaImage> {
    let mut gray_gif = Vec::new();

    for image in frames {
        let (width, height) = image.dimensions();
        let mut gray_img = RgbaImage::new(width, height);

        for (x, y, pixel) in image.enumerate_pixels() {
            let luma = (0.2126 * pixel[0] as f32
                      + 0.7152 * pixel[1] as f32
                      + 0.0722 * pixel[2] as f32) as u8;
            gray_img.put_pixel(x, y, Rgba([luma, luma, luma, pixel[3]]));
        }

        gray_gif.push(gray_img);
    }

    gray_gif
}

#[allow(unused)]
fn frames_to_ascii(frames: Vec<RgbaImage>, gradient: &Vec<String>) {
        let sleep = 100; // ms
    loop{
        for img in &frames {
            let (_width, height) = img.dimensions();
            let mut ascii_frame = String::new();

            for row in img.rows() {
                for pixel in row {
                    let mut chara: &str = &gradient[pixel[0] as usize];
                    if pixel[3] == 0 {
                        chara = " ";
                    }
                    ascii_frame.push_str(chara);
                }
                ascii_frame.push('\n');
            }

            print!("{}", &ascii_frame);
            stdout().flush().unwrap(); 
            print!("\x1B[{}A", height);
            thread::sleep(Duration::from_millis(sleep));
        }
    }
}
