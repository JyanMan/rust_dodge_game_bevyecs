use bevy_ecs::prelude::*;
use crate::core::*;
use crate::resources::*;
use crate::components::*;

pub fn spawn_text(world: &mut World, renderer: &mut Renderer, text: &str, size: i32, pos: Vector2) -> Entity {
    world.spawn((
        Transform::new(pos.x, pos.y),
        TextObject::new(renderer, text, size, pos),
    )).id()
}
