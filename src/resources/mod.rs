pub mod camera;
pub mod chunk;
pub mod chunk_manager;
pub mod asset_manager;
pub mod area_manager;
pub mod tile;
pub mod mouse_input;
pub mod entity_quad_map;
pub mod tag_registry;
pub mod draw_buffer;

pub use camera::*;
pub use chunk_manager::*;
pub use asset_manager::*;
pub use area_manager::*;
pub use mouse_input::*;
pub use entity_quad_map::*;
pub use tag_registry::*;
pub use draw_buffer::*;

use bevy_ecs::prelude::*;
use std::collections::HashSet;
use sdl2::keyboard::*;

#[derive(Resource, Default)]
pub struct DeltaTime(pub f32);

#[derive(Resource, Default)]
pub struct TimeStep(pub f32);

#[derive(Resource, Default)]
pub struct KeyInput(pub HashSet<Keycode>);
