use image::imageops::FilterType;
use image::{GenericImageView, DynamicImage};
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, ImageBuffer, Rgba, RgbaImage, Frame};
use serde::{Deserialize, Serialize};
use std::{thread, time::Duration ,env, fs, fs::File, io::BufReader, io::{stdout, Write}};
use crossterm::terminal::{size};

#[derive(Debug, Deserialize)]
struct Config {
    gradient: Vec<String>,
}

fn main() {
    // gradient from more to less bright
    let json = fs::read_to_string("config.json").unwrap();
    let config: Config = serde_json::from_str(&json).expect("Json error");
    let gradient: Vec<String> = config.gradient;
    let (term_width, term_height) = size().unwrap(); 
    //print_gradient(&gradient);
    //img_to_ascii("miku.png", 200, 200, &gradient);
    //animate_gradient(&gradient);
    
    frames_to_ascii(gif_to_gray(resize_gif(get_gif_frames("miku_dance.gif"),term_width as u32, term_height as u32)), &gradient);
}

fn print_gradient(gradient: &Vec<String>) {
    for _ in 0..5 {
        for char in gradient {
            print!("{}{}", char, char);
        }
        println!();
    }
}

fn scale_image(image_name: &str, width: u32, height: u32) {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(image_name);
    let img =
        image::open(image_name)
            .expect("No such image")
            .resize(width, height, FilterType::Lanczos3);
    img.save("output.png").unwrap();
}

fn image_to_gray(image_name: &str) {
    let img = image::open(image_name).unwrap().to_rgba8();
    let (width, height) = img.dimensions();

    let mut gray_img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let luma =
            (0.2126 * pixel[0] as f32 + 0.7152 * pixel[1] as f32 + 0.0722 * pixel[2] as f32) as u8;
        gray_img.put_pixel(x, y, Rgba([luma, luma, luma, pixel[3]]));
    }

    gray_img.save("gray.png").unwrap();
}

fn gray_to_ascii(image_name: &str, chars: &Vec<String>) {
    let step: usize = 255 / chars.len();
    let img = image::open(image_name).expect("No such image").to_rgba8();
    for (_y, row) in img.rows().enumerate() {
        for (_x, pixel) in row.enumerate() {
            let mut index = (pixel[0] as usize) / step;
            if index >= chars.len() {
                index = chars.len() - 1
            }
            let mut chara: &str = &chars[index];
            if pixel[3] == 0 {
                chara = " ";
            }
            print!("{}", chara);
        }
        println!();
    }
}

fn img_to_ascii(image_name: &str, width: u32, height: u32, gradient: &Vec<String>) {
    scale_image(image_name, width, height);
    image_to_gray("output.png");
    gray_to_ascii("gray.png", gradient);
}

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

fn resize_gif(frames: Vec<RgbaImage>, new_width: u32, new_height: u32) -> Vec<RgbaImage> {
    frames.into_iter()
        .map(|frame| {
            let dyn_img = DynamicImage::ImageRgba8(frame);
            dyn_img.resize(new_width, new_height, FilterType::Lanczos3).to_rgba8()
        })
        .collect()
}

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


fn frames_to_ascii(frames: Vec<RgbaImage>, gradient: &Vec<String>) {
        let sleep = 100; // ms
        let step: usize = 255 / gradient.len();
    loop{
        for img in &frames {
            let (_width, height) = img.dimensions();
            let mut ascii_frame = String::new();

            for row in img.rows() {
                for pixel in row {
                    let mut index = (pixel[0] as usize) / step;
                    if index >= gradient.len() {
                        index = gradient.len() - 1;
                    }

                    let mut chara: &str = &gradient[index];
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
