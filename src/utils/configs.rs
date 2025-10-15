use once_cell::sync::Lazy;
use std::{fs};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub gradient: Vec<String>,
    pub marching_squares_layers: u8,
    pub default_frame_delay: u64,
}

fn read_config() -> Config {
    let config_file =  &fs::read_to_string("config.json").expect("Error: missing json config");
    serde_json::from_str(&config_file).expect("Invalid Json")
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    read_config()
});

pub static LOOKUP: Lazy<([String; 256], usize)> = Lazy::new(|| {
    let chars: &Vec<String> = &CONFIG.gradient;

    let mut table: [String; 256] = array_init::array_init(|_| String::new());
    for i in 0..256 {
        let mut index = i * chars.len() / 256;
        if index >= chars.len() {
            index = chars.len() - 1;
        }
        table[i] = chars[index].clone();
    }
    (table, chars.len())
});