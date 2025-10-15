use crate::utils::{img_utils::*, video_utils::move_cursor_to_top_image};
use crate::utils::video_utils::frame_to_dynamic_image;
use std::thread;
use std::time::Duration;
use video_rs::decode::Decoder;
use image::{DynamicImage};
use crate::utils::lookup_table::{LOOKUP};

#[allow(unused)]
pub fn video_to_ascii(decoder: &mut Decoder, width: u32, height: u32, sleep_millis: u64) {

    loop {
        match decoder.decode() {
            Ok((_, frame)) => {
                let mut dimage = frame_to_dynamic_image(&frame);
                image_to_ascii(&mut dimage);
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

#[allow(unused)]
pub fn image_to_ascii(image: &mut DynamicImage) {

    let gray_image = simd_gray_image(image).to_rgba8();
    let width = gray_image.width() as usize;
    let height = gray_image.height() as usize;
    let mut ascii = String::with_capacity(width * height + height);

    for y in 0..height {
        for x in 0..width {
            let pixel = gray_image.get_pixel(x as u32, y as u32);
            let gray = pixel[0];
            let c = if pixel[3] == 0 { " " } else { &LOOKUP.0[gray as usize] };
            ascii.push_str(c);
        }
        ascii.push('\n');
    }

    print!("{}", ascii);
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}
