use crate::cli_interface::handle_args;

mod cli_interface;
mod filters;
mod utils;
mod media;

fn main() {
    handle_args();
}


// todo: add filters to image such as distortion

