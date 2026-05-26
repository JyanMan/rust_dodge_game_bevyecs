use crate::components::Vector2;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct PrevPos {
    pub num_frames_delay: i32,
    pub pos: Vector2,
}
