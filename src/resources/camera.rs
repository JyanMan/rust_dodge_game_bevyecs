use bevy_ecs::prelude::*;

use crate::math_helper::*;
use crate::components::{ Vector2 };
use crate::config::*;

#[derive(Resource, Default)]
pub struct Camera {
   target_pos: Vector2,
   pos: Vector2,
   pub scale: f32,
   interp: f32
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: Vector2::new(0.0, 0.0),
            target_pos: Vector2::new(0.0, 0.0),
            scale: 2.0,
            interp: 0.2,
        }
    }

    pub fn set_target(&mut self, target_pos: Vector2) {
        let screen_center = Vector2::new(HALF_WIDTH_F, HALF_HEIGHT_F) / self.scale;
        self.target_pos = target_pos - screen_center;
    }

    pub fn update(&mut self) {
        self.pos = lerp_pos(&self.pos, &self.target_pos, self.interp);
        // self.pos = self.target_pos;
    }
    pub fn get_pos(&self) -> Vector2 {
        self.pos
    }
}
