use bevy_ecs::prelude::*;

use sdl2::EventPump;
use sdl2::keyboard::*;
use crate::components::entity::{PlayerInput};
use crate::components::Combat;
use crate::resources::*;
use std::collections::HashSet;

pub fn player_input_update(input: &mut PlayerInput, k_state: &HashSet<Keycode>, combat: &mut Combat) {
    if k_state.contains(&Keycode::Space) { input.jump = true; } else { input.jump = false; }

    if k_state.contains(&Keycode::A) { input.left = true; println!("pressed a") } else { input.left = false; }

    if k_state.contains(&Keycode::D) { input.right = true; } else { input.right = false; }

    if k_state.contains(&Keycode::Q) { input.dodge = true; } else { input.dodge = false; }

    if k_state.contains(&Keycode::E) { combat.attacking = true } else { combat.attacking = false }
}

pub fn player_input_sys(input: &mut PlayerInput, event: &mut EventPump, combat: &mut Combat) {
    let k_state = event.keyboard_state();

    if k_state.is_scancode_pressed(Scancode::Space) { input.jump = true; } else { input.jump = false; }

    if k_state.is_scancode_pressed(Scancode::A) { input.left = true; } else { input.left = false; }

    if k_state.is_scancode_pressed(Scancode::D) { input.right = true; } else { input.right = false; }

    if k_state.is_scancode_pressed(Scancode::Q) { input.dodge = true; } else { input.dodge = false; }

    if k_state.is_scancode_pressed(Scancode::E) { combat.attacking = true } else { combat.attacking = false }
}
