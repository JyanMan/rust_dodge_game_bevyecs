use crate::math_helper::*;

pub struct Camera {
   target_pos: Vector2,
   pos: Vector2,
   interp: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vector2::new(0.0, 0.0),
            target_pos: Vector2::new(0.0, 0.0),
            interp: 1.0,
        }
    }

    pub fn set_target(&mut self, target_pos: &Vector2) {
        self.target_pos = target_pos.clone();
    }

    pub fn update(&mut self) {

    }
    pub fn get_pos(&self) -> Vector2 {
        self.pos.clone()
    }
}
