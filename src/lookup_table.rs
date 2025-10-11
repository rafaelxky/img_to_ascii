use once_cell::sync::Lazy;
use std::{fs, sync::Mutex};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config {
    gradient: Vec<String>,
}

pub static LOOKUP: Lazy<[String; 256]> = Lazy::new(|| {
    let config_file =  &fs::read_to_string("config.json").expect("Error: missing json config");
    let config: Config = serde_json::from_str(&config_file).expect("Invalid Json");
    let chars: Vec<String> = config.gradient;

    let mut table: [String; 256] = array_init::array_init(|_| String::new());
    for i in 0..256 {
        let mut index = i * chars.len() / 256;
        if index >= chars.len() {
            index = chars.len() - 1;
        }
        table[i] = chars[index].clone();
    }
    table
});