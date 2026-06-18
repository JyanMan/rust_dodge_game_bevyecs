use bevy_ecs::prelude::*;
use std::collections::VecDeque;
use sdl2::rect::Point;

pub enum DrawCommand {
    Line(Point, Point),
}

#[derive(Resource)]
pub struct DrawBuffer {
    pub commands: VecDeque<DrawCommand>
}
