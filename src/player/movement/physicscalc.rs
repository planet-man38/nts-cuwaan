use macroquad::prelude::Vec2;

pub fn calculate_physics(velocity: Vec2, drag: Vec2, gravity: f32, friction: f32, grounded: bool) -> Vec2 {
    let mut fin_vel: Vec2 = velocity;
    if grounded {
        fin_vel.x = fin_vel.x/friction;
        fin_vel.y = fin_vel.y/drag.y;
    } else {
        fin_vel = fin_vel/drag;
        fin_vel.y += gravity;
    }

    fin_vel
}