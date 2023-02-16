use macroquad::{prelude::*, text as macrotext};

pub fn draw_debug_text(font: Font) {
    draw_text_ex("debug enabled", 5.0, 20.0, macrotext::TextParams{font, font_size: 20, color: RED, ..Default::default()});
}