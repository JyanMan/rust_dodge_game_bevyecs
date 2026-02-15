use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::*;
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use static_cell::StaticCell;
use std::time::Duration;

use sdl2::render::*;
use sdl2::video::WindowContext;

mod game;
mod config;
// mod structs;
mod core;
mod systems;
mod resources;
mod components;
mod math_helper;
mod tests;
mod events;
mod plugins;

use crate::game::*;
use crate::plugins::*;
use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::resources::Camera;
use crate::config::*;

use crate::components::Vector2;
use crate::core::renderer::*;
use crate::resources::*;
use crate::systems::world::*;
use crate::systems::render::*;
use crate::systems::*;
use crate::config::*;

pub fn main() {
    App::new()
        // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0/60.0))))
        .add_plugins(SDLInit)
        .add_plugins(Test)
        .add_plugins(MainGame)
        .set_runner(custom_runner)
        .run();
    

}
