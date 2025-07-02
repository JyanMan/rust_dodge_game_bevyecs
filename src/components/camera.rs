use crate::math_helper::*;
use crate::components::position::*;
use crate::config::*;

#[derive(Default)]
pub struct Camera {
   target_pos: Position,
   pos: Position,
   pub scale: f32,
   interp: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Position::new(0.0, 0.0),
            target_pos: Position::new(0.0, 0.0),
            scale: 2.0,
            interp: 0.4,
        }
    }

    pub fn set_target(&mut self, target_pos: &Position) {
        let screen_center = Position::new(HALF_WIDTH_F, HALF_HEIGHT_F) / self.scale;
        self.target_pos = target_pos.clone() - screen_center;
    }

    pub fn update(&mut self) {
        self.pos = lerp_pos(&self.pos, &self.target_pos, self.interp);
        // self.pos = self.target_pos;
    }
    pub fn get_pos(&self) -> Position {
        self.pos.clone()
    }
}
