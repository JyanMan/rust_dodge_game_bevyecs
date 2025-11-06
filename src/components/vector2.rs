use std::ops::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32, pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn len(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let mut len = self.len();
        if len == 0.0 {
            len = 0.0001;
        }
        Vector2 {
            x: self.x / len,
            y: self.y / len
        }
    }

    pub fn dot(&self, other: Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

// Arithmetic
impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl <'a> Add<&'a Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, other: &'a Vector2) -> Vector2 {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl <'a> Add<Vector2> for &'a Vector2 {
    type Output = Vector2;
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl <'a> Sub<&'a Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, other: &'a Vector2) -> Vector2 {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl <'a> Sub<Vector2> for &'a Vector2 {
    type Output = Vector2;
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2 { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 { x: self.x * scalar, y: self.y * scalar }
    }
}
impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, scalar: f32) -> Vector2 {
        Vector2 { x: self.x / scalar, y: self.y / scalar }
    }
}

