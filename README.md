Gradient_ascii is a program that allows users to transforms and output multiple forms of media.
Originally it just transformed image to ascii art from a gradient ence the name, but now it supports multiple media types.

## Supported media
- local image
- local video
- http(s) image
- http(s) video

When you provide video from http(s) you must provide the url of just the media (ending in .png or .mp4 for example) else it will not work.

## Cli interface
GA uses a cli interface implemented with clap, wich includes all the default cli flags such as -h.
You can use it like:
ga <filename> <width> <height>
You can use -h to see available flags and options.
Further customization can be donne in config.json

## Config
The config.json file can be used to further customize your output.
This file is watched and provides hotswap.
This means that you can change the configuration and see its effects at runtime.
You can also use the flag -s to change configs trough the cli wich will lock that value and not allow its change at runtime
You can use -s like:
-s "varname=value"
were varname corresponds to a config variable.

## Filters
GA allows you to apply filters to the media, these filters can be chained.
Can be found under src/filters/fiters.rs
List of filters:
- rotate90
- rotate180
- blur
- gray
- invert_color
- wave
- color

## Output
Media outputs define how the result will be displayed.
Currently only terminal text output is supported.
Can be found under src/media/media_output.rs

### Output types:
#### ascii
This is the basic and original output type.
It grays the image and gets the corresponding ascii character from a lookup table corresponding to its darkness value.
You can change the gradient and its selection in the config file (gradients and selected_gradient).
This is meant to be used with a light to dark ascii gradient ex:(. + #) but works with any array of string

#### colored ascii 
This is the same as ascii except that it also colors the text to its original color.
Note that some terminal may not support this.

#### marching squares ascii
Uses marching squares algorithm to display an outline.
It grays the image and uses gray thresholds to apply marching squares.
You can select the layer ammout in the config (marching_squares_layers).

#### text color ascii 
Outputs media as colored text, however, unlike colored ascii, it doesnt use a gradient.
Ratter it outputs the exact text but colored to the media.
If the text is short, it will repeat.
if its long it will get cut.
The config is the same as the ascii.

## Media processors
Media processors controll how to apply the filters and output the media.
This is part of implementation details and is not defined by the user, but instead chosen automatically by the program.
Can be found under src/meida/media_process.rs

### Types of media processors

#### process_image
Simple processor that applies the filters and outputs the media on a single image.
Is applied if the media is detected as an image.

#### process_video
Applies the filters, outputs each frame and moved the cursor up, then increments a global frame_counter.
Loops the video when it reaches its end.
Is applied if the media is detected as a video.

## Media souces
Media sources define how to provide the media and are infered by the program from the provided media url/path.
These are part of implementation details and are not pertinent to users.
These match the supported media types so theres one for video ,audio and the http(s) counterparts.
Can be found under src/media/media_souce.rs

## Media processor
The media processor is the central authority that implements the strategy and builder patterns to operate on media.
This is part of implementation details.
Functions are provided to it, then you can call execute to call on the respective functions.
