use std::{io::BufWriter, thread, time::Duration};
use image::{DynamicImage, GenericImageView};
use video_rs::{Decoder};
use crate::utils::{lookup_table::LOOKUP, video_utils::{frame_to_dynamic_image, move_cursor_to_top_image}};
use crate::utils::img_utils::pixel_to_gray;
use std::io::Write;


pub fn image_to_colored_ascii (image: &mut DynamicImage) {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut buffer = BufWriter::new(std::io::stdout());

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel( x as u32, y as u32);
            let gray = pixel_to_gray(&pixel);
            let c = if gray == 0 { " " } else { &LOOKUP.0[gray as usize] };
            write!(buffer, "\x1b[38;2;{};{};{}m{}\x1b[0m",pixel[0],pixel[1],pixel[2],c).unwrap();
        }
        writeln!(buffer).unwrap();
    }

    buffer.flush().unwrap();
} 

#[allow(unused)]
pub fn video_to_colored_ascii(decoder: &mut Decoder, sleep_millis: u64) {

    loop {
        match decoder.decode() {
            Ok((_, frame)) => {
                let mut dimage = frame_to_dynamic_image(&frame);
                image_to_colored_ascii(&mut dimage);
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

