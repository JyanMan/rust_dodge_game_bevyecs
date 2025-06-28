use std::ops;

#[derive(Debug, Copy, Clone, Default)]
pub struct Position {
    pub x: f32, pub y: f32
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x: x, y: y}
    }
}

impl ops::Mul<f32> for Position {
    type Output = Position;

    fn mul(self, other: f32) -> Position {
        Position {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl ops::Mul<Position> for Position {
    type Output = Position;

    fn mul(self, other: Position) -> Position {
        Position {
            x: self.x * other.x,
            y: self.y * other.y
        }
    }
}

impl ops::Add<Position> for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
} 

impl ops::Add<&Position> for Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
} 

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
} 

impl<'a> ops::Sub<&'a Position> for Position {
    type Output = Position;

    fn sub(self, rhs: &'a Position) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a> ops::Sub<Position> for &'a Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Position> for &'a Position {
    type Output = Position;

    fn sub(self, rhs: &'b Position) -> Position {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
