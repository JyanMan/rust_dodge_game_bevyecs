use sdl2::keyboard::*;
use crate::components::entity::{PlayerInput};

pub fn player_input_sys(input: &mut PlayerInput, k_state: &mut KeyboardState) {
    if k_state.is_scancode_pressed(Scancode::Space) { input.jump = true; } else { input.jump = false; }

    if k_state.is_scancode_pressed(Scancode::A) { input.left = true; } else { input.left = false; }

    if k_state.is_scancode_pressed(Scancode::D) { input.right = true; } else { input.right = false; }

    if k_state.is_scancode_pressed(Scancode::Q) { input.dodge = true; } else { input.dodge = false; }
}
