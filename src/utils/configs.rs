use clap::Parser;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{fs};
use serde::{Deserialize, Serialize};

use crate::cli_interface::Args;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub gradients: Vec<Vec<String>>,
    pub marching_squares_layers: u8,
    pub default_frame_delay: u64,
    pub selected_gradient: usize,
    pub blur_sigma: f32,
}

fn read_config() -> Config {
    let config_file =  &fs::read_to_string("config.json").expect("Error: missing json config");
    serde_json::from_str(&config_file).expect("Invalid Json")
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut value = serde_json::to_value(read_config()).unwrap();

    for kv in &ARGS.set {
        if let Some((key, val)) = kv.replace(' ', "").split_once('=') {
            let val_json = serde_json::from_str(val).unwrap_or(Value::String(val.to_string()));
            value[key] = val_json;
        } 
    }

    serde_json::from_value(value).unwrap()
});

pub static ARGS: Lazy<Args> = Lazy::new(|| {
    Args::parse()
});

pub static LOOKUP: Lazy<([String; 256], usize)> = Lazy::new(|| {
    let chars: &Vec<String> = &CONFIG.gradients[CONFIG.selected_gradient];

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

