mod game;
mod config;
mod core;
mod systems;
mod resources;
mod components;
mod math_helper;
mod tests;
mod events;
mod plugins;

use bevy_app::prelude::*;
use crate::plugins::*;
use crate::core::renderer::*;

pub fn main() {
    App::new()
        .add_plugins(SDLInit)
        .add_plugins(Test)
        .add_plugins(MainGame)
        .set_runner(custom_runner)
        .run();
}
