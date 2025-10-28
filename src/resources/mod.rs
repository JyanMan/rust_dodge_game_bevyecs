pub mod camera;
pub mod chunk;
pub mod chunk_manager;
pub mod asset_manager;
pub mod area_manager;
pub mod tile;
pub mod mouse_input;
pub mod register_resources;

pub use camera::*;
pub use chunk::*;
pub use chunk_manager::*;
pub use asset_manager::*;
pub use area_manager::*;
pub use tile::*;
pub use mouse_input::*;

use bevy_ecs::prelude::*;
use std::collections::HashSet;
use sdl2::keyboard::*;

#[derive(Resource)]
pub struct DeltaTime(pub f32);

#[derive(Resource)]
pub struct TimeStep(pub f32);

#[derive(Resource, Default)]
pub struct KeyInput(pub HashSet<Keycode>);
