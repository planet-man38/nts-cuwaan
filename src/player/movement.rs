use super::positions::{Direction};
use macroquad::prelude::*;

pub mod checkcollision;
pub mod physicscalc;

const GRAVITY: f32 = 1.5;
const AIRSPEED: f32 = 1.5;
const SPEED: f32 = 20.0;
const DRAG: Vec2 = Vec2{x: 1.1, y: 1.01};
const JUMP_STRENGTH: f32 = 30.0;
const FRICTION: f32 = 1.6;

pub fn calculate_new_velocity(velocity: Vec2, obstacles: &[Rect], player: &mut Rect) -> Vec2 {
    let mut new_vel = velocity;

    let mut grounded: bool = false;
    let mut new_vel_override: Vec2 = Vec2{x: 1.0, y: 1.0};
    for i in obstacles {
        let collision = checkcollision::check_collision(*player, *i);

        if collision.collided {
            if collision.movement.y == 0.0 {
                player.x -= collision.movement.x;
                new_vel_override.x = 0.0;
            }
            if collision.movement.x == 0.0 {
                player.y -= collision.movement.y;
                new_vel_override.y = 0.0;
            }

            if collision.direction == Direction::Down {
                grounded = true;
            }
        }
        println!("{}", collision.movement);
    }

    let mut new_vel_add = Vec2{x: 0.0, y: 0.0};
    
    resolve_movement(Direction::Right, &[KeyCode::D, KeyCode::Right], &mut new_vel_add, if grounded {SPEED} else {AIRSPEED});
    resolve_movement(Direction::Left, &[KeyCode::A, KeyCode::Left], &mut new_vel_add, if grounded {SPEED} else {AIRSPEED});
    resolve_jump(&[KeyCode::W, KeyCode::Space, KeyCode::Up], &mut new_vel, JUMP_STRENGTH);
    new_vel += new_vel_add;

    new_vel = physicscalc::calculate_physics(new_vel, DRAG, GRAVITY, FRICTION, grounded);

    return Vec2{x: new_vel.x * new_vel_override.x, y: new_vel.y * new_vel_override.y}
}

fn resolve_movement(direction: Direction, keys: &[KeyCode], velocity: &mut Vec2, speed: f32) {
    for i in keys{
        if is_key_down(*i) {
            match direction {
                Direction::Up => velocity.y = -speed,
                Direction::Down => velocity.y = speed,
                Direction::Right => velocity.x = speed,
                Direction::Left => velocity.x = -speed
            }
        }
    }
}
fn resolve_jump(keys: &[KeyCode], velocity: &mut Vec2, strength: f32) {
    for i in keys{
        if is_key_pressed(*i) {
            velocity.y = -strength;
        }
    }
}
