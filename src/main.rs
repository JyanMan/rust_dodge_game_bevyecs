mod config;
mod core;
mod sys;
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
        .add_plugins(MainGame)
        .add_plugins(Physics2D)
        .add_plugins(Timers)
        .add_plugins(States)
        .add_plugins(Test)
        .set_runner(custom_runner)
        .run();
}
