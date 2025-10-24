use crate::{cli_interface::handle_args, utils::configs::watch_config};

mod cli_interface;
mod filters;
mod utils;
mod media;

fn main() {
    watch_config().expect("Error watching file!");
    handle_args();
}


// todo: add filters to image such as distortion

