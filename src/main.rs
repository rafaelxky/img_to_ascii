mod cli_interface;
mod facade;
mod gradient;
mod filters;
mod utils;
use cli_interface::*;

use crate::{media_processor::MediaProcessor, utils::img_utils::get_image};
use crate::filters::ascii::image_to_ascii;
mod media_processor;

fn main() {
    handle_args();
    let mp = MediaProcessor::new("miku.png".to_string())
    .with_get_image(&get_image)
    .with_process_image(&image_to_ascii)
    .execute();
}

