mod cli_interface;
mod filters;
mod utils;
mod media_type;
use std::path::Path;

use cli_interface::*;
use video_rs::{ffmpeg::filter::Source, Decoder};

use crate::{media_type::MediaType, utils::video_utils::get_video_decoder};

mod media_processor;

fn main() {
    handle_args();
}


// todo: add filters to image such as distortion

