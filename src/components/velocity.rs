use std::ops::*;
use crate::components::position::Position;

#[derive(Debug, Copy, Clone, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0.0 , y: 0.0  }
    }
}

// Arithmetic
impl Add<Velocity> for Position {
    type Output = Position;
    fn add(self, vel: Velocity) -> Position {
        Position { x: self.x + vel.x, y: self.y + vel.y }
    }
}
impl Add<Velocity> for Velocity {
    type Output = Velocity;
    fn add(self, other: Velocity) -> Velocity {
        Velocity { x: self.x + other.x, y: self.y + other.y }
    }
}
impl Mul<f32> for Velocity {
    type Output = Velocity;
    fn mul(self, scalar: f32) -> Velocity {
        Velocity { x: self.x * scalar, y: self.y * scalar }
    }
}

