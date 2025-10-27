use std::char;
use std::io::{BufWriter, Write};
use image::{DynamicImage, GenericImageView};

use crate::utils::configs::*;
use crate::utils::img_utils::{pixel_to_gray};
use crate::utils::marching_squares_utils::get_marching_squares_case;

pub fn ascii_output(image: &mut DynamicImage){

    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut ascii = String::with_capacity(width * height + height);

    for y in 0..height {
        for x in 0..width { 
            let pixel = &image.get_pixel(x as u32, y as u32); 
            if pixel[3] == 0 {
                ascii.push(' ');
                continue;
            }
            let gray = pixel_to_gray(pixel);
            let c = &get_lookup().0[gray as usize];
            ascii.push_str(c);
        }
        ascii.push('\n');
    }

    print!("{}", ascii);
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}

pub fn colored_ascii_output (image: &mut DynamicImage) {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut buffer = BufWriter::new(std::io::stdout());

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel( x as u32, y as u32);
            if pixel[3] == 0 {
                write!(buffer, " ").unwrap();
                continue;
            }
            let gray = pixel_to_gray(&pixel);
            let c = &get_lookup().0[gray as usize];
            write!(buffer, "\x1b[38;2;{};{};{}m{}\x1b[0m",pixel[0],pixel[1],pixel[2],c).unwrap();
        }
        writeln!(buffer).unwrap();
    }
    buffer.flush().unwrap();
}

pub fn marching_squares_ascii_output(image: &mut DynamicImage){
    let tolerance = get_config().marching_squares_layers;
    let image_buffer = image.to_luma8();
    let mut result = String::with_capacity((image_buffer.height() * (image_buffer.width() + 1)) as usize);

    for y in 0..image_buffer.height() - 1 {
        for x in 0..image_buffer.width() - 1{

            let tl = image_buffer.get_pixel(x, y)[0]; 
            let tr = image_buffer.get_pixel(x + 1, y)[0]; 
            let bl = image_buffer.get_pixel(x, y + 1)[0]; 
            let br = image_buffer.get_pixel(x + 1, y + 1)[0]; 

            result.push_str(&get_marching_squares_case(tl, tr, bl, br, tolerance));
        }
        result.push('\n');
    }
    println!("{}", result);
}

pub fn text_color_ascii_output (image: &mut DynamicImage) {
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut buffer = BufWriter::new(std::io::stdout());

    let gradient_str = &get_config().gradients[get_config().selected_gradient].join("");
    let gradient_chars: Vec<char> = gradient_str.chars().collect();
    let gradient_len = gradient_chars.len();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel( x as u32, y as u32);
            if pixel[3] == 0 {
                write!(buffer, " ").unwrap();
                continue;
            }
            let c: char = gradient_chars[((width * y) + x) % gradient_len];
            write!(buffer, "\x1b[38;2;{};{};{}m{}\x1b[0m",pixel[0],pixel[1],pixel[2],c).unwrap();
        }
        writeln!(buffer).unwrap();
    }
    buffer.flush().unwrap();
}