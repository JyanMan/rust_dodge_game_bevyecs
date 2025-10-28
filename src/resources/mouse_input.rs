use bevy_ecs::prelude::*;

use crate::components::Vector2;
use crate::config::*;

#[derive(Resource, Default, Clone)]
pub struct MouseInput {
    pub pos: Vector2,
}

impl MouseInput {
    pub fn dir_from_center(&self) -> Vector2 {
        let mouse_pos = self.pos;
        let screen_center = Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);

        // get mouse direction and distance from screen center
        let mouse_delta = mouse_pos - screen_center;
        let mouse_dir = mouse_delta.normalize();
        mouse_dir
    }

    pub fn dist_to_center(&self) -> Vector2 {
        let mouse_pos = self.pos;
        let screen_center = Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);

        // get mouse direction and distance from screen center
        let mouse_delta = mouse_pos - screen_center;
        mouse_delta
    }
}
