use std::{thread, time::Duration};
use image::{DynamicImage};
use video_rs::Decoder;

use crate::video::frame_to_dynamic_image;

#[allow(unused)]
pub fn video_to_marching_squares(decoder: &mut Decoder, width: u32, height: u32, sleep_millis: u64, tolerance: u8) {

    loop {
        match decoder.decode() {
            Ok((_, frame)) => {
                image_to_marching_squares_ascii(&mut frame_to_dynamic_image(&frame), tolerance);
                print!("\x1B[{}A", height);
            }
            Err(video_rs::Error::DecodeExhausted) => {
                decoder.seek_to_start().unwrap();
            }
            Err(e) => {
                eprintln!("Decode error: {:?}", e);
                break;
            }
        }
        thread::sleep(Duration::from_millis(sleep_millis));
    }
}

// image is already gray
pub fn image_to_marching_squares_ascii(image: &DynamicImage, tolerance: u8){
    let image = image.to_luma8();
    let mut result = String::with_capacity((image.height() * (image.width() + 1)) as usize);

    for y in 0..image.height() - 1 {
        for x in 0..image.width() - 1{

            let tl = image.get_pixel(x, y)[0]; 
            let tr = image.get_pixel(x + 1, y)[0]; 
            let bl = image.get_pixel(x, y + 1)[0]; 
            let br = image.get_pixel(x + 1, y + 1)[0]; 

            result.push_str(&get_case(tl, tr, bl, br, tolerance));
        }
        result.push('\n');
    }
    println!("{}", result);
}

pub fn get_case(tl: u8, tr: u8, bl: u8, br: u8, tolerance: u8) -> String {
    // Helper: are two corners within tolerance of each other?
    let similar = |a: u8, b: u8| -> bool {
        if a > b { a - b <= tolerance } else { b - a <= tolerance }
    };

    // Check pairs of corners
    let top_sim = similar(tl, tr);
    let bottom_sim = similar(bl, br);
    let left_sim = similar(tl, bl);
    let right_sim = similar(tr, br);
    let diag1 = similar(tl, br); // top-left -> bottom-right
    let diag2 = similar(tr, bl); // top-right -> bottom-left

    // Decide character
    if top_sim && bottom_sim && left_sim && right_sim {
        " ".to_string() // all similar
    } else if top_sim && bottom_sim {
        "-".to_string() // horizontal edges
    } else if left_sim && right_sim {
        "|".to_string() // vertical edges
    } else if diag1 {
        "\\".to_string() // diagonal TL -> BR
    } else if diag2 {
        "/".to_string() // diagonal TR -> BL
    } else {
        "-".to_string() // fallback
    }
}
