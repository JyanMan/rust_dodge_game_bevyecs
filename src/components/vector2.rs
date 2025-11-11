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
    
    pub fn rotate_around(&self, center: Vector2, angle_rad: f32) -> Vector2 {

        let x = self.x - center.x;
        let y = self.y - center.y;

        let s = angle_rad.sin();
        let c = angle_rad.cos();

        let x_rot = x * c - y * s;
        let y_rot = x * s + y * c;

        Vector2 { x: x_rot + center.x, y: y_rot + center.y }
    }
// Vector2 v2_rotate_around(Vector2 point, Vector2 center, float angle_rad) {
//     float s = sinf(angle_rad);
//     float c = cosf(angle_rad);
// 
//     // Translate to center
//     float x = point.x - center.x;
//     float y = point.y - center.y;
// 
//     // Rotate
//     float x_rot = x * c - y * s;
//     float y_rot = x * s + y * c;
// 
//     // Translate back
//     return (Vector2){ x_rot + center.x, y_rot + center.y };
// }
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

impl Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 { x: &self.x * scalar, y: &self.y * scalar }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, scalar: f32) -> Vector2 {
        Vector2 { x: self.x / scalar, y: self.y / scalar }
    }
}

