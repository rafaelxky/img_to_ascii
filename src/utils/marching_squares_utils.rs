
pub fn get_marching_squares_case(tl: u8, tr: u8, bl: u8, br: u8, layers: u8) -> String {
    let layer_size = 256 / (layers + 1) as u16; 

    let same_layer = |a: u8, b: u8| -> bool {
        (a as u16 / layer_size) == (b as u16 / layer_size)
    };

    let top_sim = same_layer(tl, tr);
    let bottom_sim = same_layer(bl, br);
    let left_sim = same_layer(tl, bl);
    let right_sim = same_layer(tr, br);
    let diag1 = same_layer(tl, br);
    let diag2 = same_layer(tr, bl);

    if top_sim && bottom_sim && left_sim && right_sim {
        " ".to_string() 
    } else if top_sim && bottom_sim {
        "-".to_string()
    } else if left_sim && right_sim {
        "|".to_string()
    } else if diag1 {
        "\\".to_string()
    } else if diag2 {
        "/".to_string()
    } else {
        "-".to_string()
    }
}

