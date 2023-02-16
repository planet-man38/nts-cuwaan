use debug::draw_debug_text;
use macroquad::{prelude::*, text as macrotext};
use player::{movement::calculate_new_velocity};

pub mod player;
pub mod debug;

#[macroquad::main("h destroi g")]
async fn main() {
    //Arial font, our lord and saviour. Father of ✨Arial Lowercase h✨
    let arial = load_ttf_font("./fonts/arial.ttf").await.unwrap();
    //Arial black, Arial's lesser brother. Still cool font tho
    let other_arial = load_ttf_font("./fonts/ariblk.ttf").await.unwrap();

    const OBSTACLES: [Rect; 1] = [Rect{x: 500.0, y: 300.0, w: 300.0, h: 300.0}];

    //useful vars
    let mut velocity = Vec2{x: 0.0, y: 0.0};
    let mut player: Rect = Rect{x: 300.0, y: 300.0, w: 70.0, h: 70.0};
    let debug: bool = true;
    let mut pause: bool = true;

    //main loop
    loop {
        //toggle pause
        if is_key_pressed(KeyCode::Escape) {
            pause = !pause;
        }

        //calculate everything if not paused
        if !pause {
            velocity = calculate_new_velocity(velocity, &OBSTACLES, &mut player);
            player.x += velocity.x; player.y += velocity.y;
        }

        //draw everything
        clear_background(Color { r: 0.2, g: 0.6, b: 1.0, a: 1.0 });
        for i in OBSTACLES { //draw the obstacles
            draw_rectangle(i.x, i.y, i.w, i.h, GREEN);
        }
        player::draw_h(arial, player);

        //shit that needs to be overlayed over other shit
        if pause { //draw pause related stuff
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5 });
            draw_text_ex("PAUSED", screen_width() - 240.0, 50.0, macrotext::TextParams{font: other_arial, font_size: 50, color: BLACK, ..Default::default()});
        }
        if debug { //draw debug related stuff
            draw_debug_text(other_arial);
        }
        
        //await next frame (duh)
        next_frame().await
    }
}
