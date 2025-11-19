use super::Vector2;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Copy, Clone, Default, PartialEq)]
#[component(storage = "Table")]
pub struct Transform {
    pub global: Vector2,
    pub local: Vector2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            global: Vector2::new( x, y ), 
            local: Vector2::zero(),
            rotation: 0.0,
        }
    }
    pub fn zero() -> Self {
        Self { 
            global: Vector2::new( 0.0, 0.0 ), 
            local: Vector2::zero() ,
            rotation: 0.0,
        }
    }
}
// 
// // Arithmetic
// impl Add for Transform {
//     type Output = Transform;
//     fn add(self, other: Transform) -> Transform {
//         Transform { x: self.x + other.x, y: self.y + other.y }
//     }
// }
// impl Sub for Transform {
//     type Output = Transform;
//     fn sub(self, other: Transform) -> Transform {
//         Transform { x: self.x - other.x, y: self.y - other.y }
//     }
// }
// impl Mul<f32> for Transform {
//     type Output = Transform;
//     fn mul(self, scalar: f32) -> Transform {
//         Transform { x: self.x * scalar, y: self.y * scalar }
//     }
// }
// impl Div<f32> for Transform {
//     type Output = Transform;
//     fn div(self, scalar: f32) -> Transform {
//         Transform { x: self.x / scalar, y: self.y / scalar }
//     }
// }
// 
