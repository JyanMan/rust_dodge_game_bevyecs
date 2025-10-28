use bevy_ecs::prelude::*;

use sdl2::EventPump;
use sdl2::keyboard::*;
use crate::components::entity::{PlayerInput};
use crate::components::Combat;
use crate::resources::*;
use std::collections::HashSet;

pub fn player_system_input(mut query: Query<(&mut PlayerInput, &mut Combat)>, user_input_res: Res<KeyInput>) {
    for (mut input, mut combat) in &mut query {
        if user_input_res.0.contains(&Keycode::Space) { input.jump = true; } else { input.jump = false; }

        if user_input_res.0.contains(&Keycode::A) { input.left = true; } else { input.left = false; }

        if user_input_res.0.contains(&Keycode::D) { input.right = true; } else { input.right = false; }

        if user_input_res.0.contains(&Keycode::Q) { input.dodge = true; } else { input.dodge = false; }

        if user_input_res.0.contains(&Keycode::E) { combat.attacking = true } else { combat.attacking = false }
    }
}
