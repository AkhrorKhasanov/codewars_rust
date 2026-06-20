fn guess_blue(blue_start: u32, red_start: u32, blue_pulled: u32, red_pulled: u32) -> f32 {
    ((blue_start - blue_pulled) as f32) / ((red_start + blue_start - red_pulled - blue_pulled) as f32) as f32
}