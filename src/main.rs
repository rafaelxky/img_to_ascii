use crate::{cli_interface::handle_args, utils::configs::watch_config};

mod cli_interface;
mod filters;
mod utils;
mod media;

fn main() {
    match watch_config() {
        Ok(_) => {
            ()
        },
        Err(_) => {
            eprintln!("Error: could not find config file!")
        }
    }
    handle_args();
}

// todo:
// add filter chain and output option for config so you can change the behaviour at runtime
// same for width and height
// implement support for 3d
// filter like marching squares but with characters just to do outlines
