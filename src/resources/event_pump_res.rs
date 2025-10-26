use sdl2::EventPump;
use std::rc::Rc;
use std::cell::RefCell;
use bevy_ecs::prelude::*;
use sdl2::keyboard::*;
use std::collections::HashSet;

#[derive(Resource, Default)]
pub struct UserInputRes {
    pub k_state: HashSet<Keycode>,
}
