use macroquad::prelude::{Vec2, Rect};
use crate::player::positions::{Direction, closenull};

struct AABB {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32
}

pub struct Collision {
    pub collided: bool,
    pub direction: Direction,
    pub movement: Vec2
}

pub fn check_collision(b1: Rect, b2: Rect) -> Collision {
    let box1 = AABB {
        x1: b1.x,
        x2: b1.x + b1.w,
        y1: b1.y,
        y2: b1.y + b1.h
    };
    let box2 = AABB {
        x1: b2.x,
        x2: b2.x + b2.w,
        y1: b2.y,
        y2: b2.y + b2.h
    };

    let mut movement = Vec2 {
        x: closenull(box1.x1-box2.x2, box2.x1-box1.x2),
        y: closenull(box1.y1-box2.y2, box2.y1-box1.y2)
    };

    if movement.x < movement.y {
        movement.x = 0.0;
    } else if movement.y < movement.x {
        movement.y = 0.0;
    } else if movement.x == movement.y {
        movement.x = 0.0;
    }

    let mut direction: Direction = Direction::Down;

    if movement.x > 0.0 {
        direction = Direction::Left;
    }
    if movement.x < 0.0 {
        direction = Direction::Right;
    }
    if movement.y > 0.0 {
        direction = Direction::Up;
    }
    if movement.y < 0.0 {
        direction = Direction::Down;
    }
    Collision {
        collided: (box1.x1 < box2.x2) &
                  (box1.x2 > box2.x1) &
                  (box1.y1 < box2.y2) &
                  (box1.y2 > box2.y1),
        direction: direction,
        movement: movement
    }
}

