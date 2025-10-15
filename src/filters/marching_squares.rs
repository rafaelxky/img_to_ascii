use std::{thread, time::Duration};
use image::{DynamicImage};
use video_rs::Decoder;

use crate::utils::video_utils::{frame_to_dynamic_image, move_cursor_to_top_image};

#[allow(unused)]
pub fn video_to_marching_squares(decoder: &mut Decoder, sleep_millis: u64, tolerance: u8) {

    loop {
        match decoder.decode() {
            Ok((_, frame)) => {
                let mut dimage = frame_to_dynamic_image(&frame);
                image_to_marching_squares_ascii(&dimage, tolerance);
                move_cursor_to_top_image(&dimage);
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

fn get_case(tl: u8, tr: u8, bl: u8, br: u8, layers: u8) -> String {
    let layer_size = 256 / (layers + 1) as u16; 

    let same_layer = |a: u8, b: u8| -> bool {
        (a as u16 / layer_size) == (b as u16 / layer_size)
    };

    let top_sim = same_layer(tl, tr);
    let bottom_sim = same_layer(bl, br);
    let left_sim = same_layer(tl, bl);
    let right_sim = same_layer(tr, br);
    let diag1 = same_layer(tl, br);
    let diag2 = same_layer(tr, bl);

    if top_sim && bottom_sim && left_sim && right_sim {
        " ".to_string() 
    } else if top_sim && bottom_sim {
        "-".to_string()
    } else if left_sim && right_sim {
        "|".to_string()
    } else if diag1 {
        "\\".to_string()
    } else if diag2 {
        "/".to_string()
    } else {
        "-".to_string()
    }
}

