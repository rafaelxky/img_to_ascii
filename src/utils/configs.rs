use clap::Parser;
use notify::EventKind;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::Arc;
use std::fs;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use once_cell::sync::Lazy;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::error::Error;
use directories::ProjectDirs;

use crate::cli_interface::Args;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub gradients: Vec<Vec<String>>,
    pub marching_squares_layers: u8,
    pub default_frame_delay: u64,
    pub selected_gradient: usize,
    pub blur_sigma: f32,
    pub wave_amplitude: f32,
    pub wave_frequency: f32,
    pub color: [i16; 3],
}

fn read_config() -> Config {
    let config_file =  &fs::read_to_string("config.json").expect("Error: missing json config");
    serde_json::from_str(&config_file).expect("Invalid Json")
}

pub fn get_config() -> Arc<Config> {
    CONFIG.read().expect("Error: could not get read on config").clone()
}

const DEFAULT_CONFIG: &str = include_str!("../../config.json");

pub static CONF_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let proj = ProjectDirs::from("me", "rafaelxky", "gradient_ascii")
        .expect("Cannot determine config directory");
    let config_dir = proj.config_dir();
    fs::create_dir_all(config_dir).expect("Error: could not create config directory!");
    let config_path = config_dir.join("config.json");
    if !config_path.exists() {
        fs::write(&config_path, DEFAULT_CONFIG).expect("Failed to write default config");
        println!("Default config.json copied to {}", config_path.display());
    }
    config_path
});

pub fn watch_config() -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(tx, notify::Config::default().with_poll_interval(Duration::from_millis(1000)))?;

    watcher.watch(&CONF_PATH, RecursiveMode::NonRecursive)?;

    std::thread::spawn(move || {
        let _watcher = watcher;
        for event in rx {
            if let Ok(ev) = event {
                if matches!(ev.kind, EventKind::Modify(_)) {
                    reload_config_loop();
                }
            }
        }
    });

    Ok(())
}

fn reload_config() -> Result<(), Box<dyn Error>> {
    let config_file =  &fs::read_to_string("config.json").expect("Error: missing json config");
    let mut config_value: Value = serde_json::from_str(config_file)?; 

   apply_config_overrides(&mut config_value);

    let new_config = serde_json::from_value(config_value)?;
    let mut cfg = CONFIG.write().expect("Error: could not obtain write to CONFIG");
    *cfg = Arc::new(new_config);
    Ok(())
}

fn reload_config_loop(){
    let mut tries = 0;
    while tries <= 100 {
        match reload_config() {
            Ok(()) => {
                reload_lookup();
            },
            Err(_) => {
                tries += 1;
            }, 
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn apply_config_overrides(config: &mut Value){
    for kv in &ARGS.set {
        if let Some((key, val)) = kv.replace(' ', "").split_once('=') {
            let val_json = serde_json::from_str(val).unwrap_or(Value::String(val.to_string()));
            config[key] = val_json;
        } 
    }
}

pub static CONFIG: Lazy<RwLock<Arc<Config>>> = Lazy::new(|| {
    let mut value = serde_json::to_value(read_config()).unwrap();

    for kv in &ARGS.set {
        if let Some((key, val)) = kv.replace(' ', "").split_once('=') {
            let val_json = serde_json::from_str(val).unwrap_or(Value::String(val.to_string()));
            value[key] = val_json;
        } 
    }

    RwLock::new(Arc::new(serde_json::from_value(value).unwrap()))
});

pub static ARGS: Lazy<Args> = Lazy::new(|| {
    Args::parse()
});

pub fn get_lookup() -> Arc<([String; 256], usize)> {
    LOOKUP.read().expect("Error: could not get read on config").clone()
}

pub static LOOKUP: Lazy<RwLock<Arc<([String; 256], usize)>>> = Lazy::new(|| {
    RwLock::new(Arc::new(build_lookup()))
});

fn build_lookup() -> ([String; 256], usize) {
    let gradients = &get_config().gradients;
    let mut selected_gradient = get_config().selected_gradient;
    if selected_gradient >= gradients.len() {
        selected_gradient = 0;
    }
    let chars: &Vec<String> = &gradients[selected_gradient];

    let mut table: [String; 256] = array_init::array_init(|_| String::new());
    for i in 0..256 {
        let mut index = i * chars.len() / 256;
        if index >= chars.len() {
            index = chars.len() - 1;
        }
        table[i] = chars[index].clone();
    }
    (table, chars.len())
}

fn reload_lookup() {
    let new_lookup = build_lookup();
    let mut lookup_write = LOOKUP.write().expect("Error: could not obtain write to LOOKUP");
    *lookup_write = Arc::new(new_lookup);
}

pub fn get_frame_counter() -> usize {
    return *FRAME_COUNTER.lock().unwrap();
}
pub static FRAME_COUNTER: Lazy<Mutex<usize>> = Lazy::new(| | Mutex::new(0));

