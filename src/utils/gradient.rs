use crate::utils::configs::{CONFIG};
use std::{thread, time::Duration};

#[allow(unused)]
fn print_gradient() {
    let gradient = &CONFIG.gradients[0];
    for _ in 0..5 {
        for char in gradient.iter() {
            print!("{}{}", char, char);
        }
        println!();
    }
}

#[allow(unused)]
fn animate_gradient() {
    let mut gradient = CONFIG.gradients[0].clone();
    let height = 5;
    let frames = 40;
    let sleep = 100;
    let reversed: Vec<String> = gradient.iter().rev().cloned().collect();
    gradient.extend(reversed);

    for f in 0..frames {
        let mut buffer = String::new();
        if f > 0 {
            buffer.push_str(&format!("\x1B[{}A", height));
        }
        for _ in 0..height {
            for i in 0..gradient.len() {
                let chara = &gradient[(f + i) % gradient.len()];
                buffer.push_str(&format!("{}{}", chara, chara));
            }
            buffer.push_str("\n");
        }
        print!("{}", buffer);
        thread::sleep(Duration::from_millis(sleep));
    }
}
