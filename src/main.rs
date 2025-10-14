use std::{thread, time::Duration};
mod img_filter;
mod video;
mod lookup_table;
mod cli_interface;
mod facade;
use cli_interface::*;

fn main() {
    handle_args();

    // gradient from less to more bright
    //let (term_width, term_height) = size().unwrap(); 
    //let gradient: Vec<String> = LOOKUP.0.to_vec();

    //let path = "video.mp4";
    //let path = "bb.mp4";
    
    //mp4, gif
    //video_to_ascii(path, term_width as u32 - 20, term_height as u32 - 20, 10);

    // image
    //image_to_ascii(&gradient, "photo2.png", 100, 100);
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
