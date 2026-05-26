use crate::components::Vector2;
use bevy_ecs::prelude::*;
use std::collections::VecDeque;

#[derive(Component, Default)]
pub struct PrevPos {
    pub num_frames_delay: i32,
    pub pos: VecDeque<Vector2>,
}
