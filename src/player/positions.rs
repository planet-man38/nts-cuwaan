#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}

pub fn closenull(num1: f32, num2: f32) -> f32 {
    let num1dist = (0.0 - num1).abs();
    let num2dist = (0.0 - num2).abs();

    if num1dist < num2dist {
        return num1
    } else {
        return num2
    }
}
