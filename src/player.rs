use macroquad::prelude::*;
use macroquad::text as macrotext;

pub mod movement;
pub mod positions;

pub fn draw_h(font: Font, player: Rect) {
    draw_rectangle(player.x, player.y, player.w, player.h, WHITE);
    draw_text_ex("h", player.x + 12.0, player.y + 65.0, macrotext::TextParams{font: font, font_size: 80, color: BLACK, ..Default::default()});
}