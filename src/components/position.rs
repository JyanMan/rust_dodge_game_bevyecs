use std::ops::*;
use super::Vector2;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Position {
    pub vec: Vector2
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { vec: Vector2::new( x, y ) }
    }
    pub fn zero() -> Self {
        Self { vec: Vector2::new( 0.0, 0.0 ) }
    }
}
// 
// // Arithmetic
// impl Add for Position {
//     type Output = Position;
//     fn add(self, other: Position) -> Position {
//         Position { x: self.x + other.x, y: self.y + other.y }
//     }
// }
// impl Sub for Position {
//     type Output = Position;
//     fn sub(self, other: Position) -> Position {
//         Position { x: self.x - other.x, y: self.y - other.y }
//     }
// }
// impl Mul<f32> for Position {
//     type Output = Position;
//     fn mul(self, scalar: f32) -> Position {
//         Position { x: self.x * scalar, y: self.y * scalar }
//     }
// }
// impl Div<f32> for Position {
//     type Output = Position;
//     fn div(self, scalar: f32) -> Position {
//         Position { x: self.x / scalar, y: self.y / scalar }
//     }
// }
// 
