use bevy_ecs::prelude::*;

use sdl2::keyboard::*;
use crate::components::entity::{PlayerInput};
use crate::resources::*;

pub fn player_system_input(mut query: Query<&mut PlayerInput>, user_input_res: Res<KeyInput>) {
    for mut input in &mut query {
        input.jump = user_input_res.0.contains(&Keycode::Space);

        input.left = user_input_res.0.contains(&Keycode::A);

        input.right = user_input_res.0.contains(&Keycode::D);

        input.dodge = user_input_res.0.contains(&Keycode::Q);

        input.attack = user_input_res.0.contains(&Keycode::E);
    }
}
