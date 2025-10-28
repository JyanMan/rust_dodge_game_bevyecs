use sdl2::EventPump;
use sdl2::keyboard::*;
use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::entity::{ WalkerData, WalkerState, WalkerAnim, };
use crate::components::*;
use crate::ecs::ecs::*;
use crate::resources::asset_manager::*;
use crate::resources::event_pump_res::UserInputRes;
use crate::resources::*;
use crate::components::entity::*;
use crate::systems::weapon::*;

use PlayerState as P;

pub fn player_timer_system(mut query: Query<(&mut PlayerData, &mut WalkerData)>, dt_res: Res<DeltaTimeRes>) {
    use super::player_timer::*;
    for (mut p_data, mut walker_d) in &mut query {
        let delta_time = dt_res.delta_time;
        player_can_jump_delay_timer(&mut p_data, &mut walker_d, delta_time);
        player_dodge_timer(&mut p_data, delta_time);
        player_lerp_timer(&mut p_data, delta_time);
    } 
}

pub fn player_movement_system(
    mut query: Query<(&mut PlayerData, &mut WalkerData, &mut Velocity, &PlayerInput, &Combat)>, 
    mouse_input: Res<MouseInput>
) {
    use super::player_movement::*;

    let mouse_pos = mouse_input.pos;

    for (mut p_data, mut walker_d, mut vel, input, combat) in &mut query {
        if input.dodge && p_data.can_dodge {
            player_dodge(&mut p_data);
        }

        // only allow dodge again if dodge button is let go
        // otherwise, player can fly like superman
        if p_data.state != P::Dodging && !input.dodge {
            p_data.can_dodge = true; 
        }
        if p_data.state == P::Dodging {
            let dodge_dir = get_dodge_dir(mouse_pos, &mut p_data);
            player_dodging(dodge_dir, &mut p_data, &mut vel);
            return;
        } 
        if p_data.state == P::Lerping {
            player_lerping(&mut vel);
            return;
        }
        if combat.attacking {
            vel.vec = vel.vec * 0.5;
            // steel_sword_movement_effect(&mut vel, mouse_input.clone());
            return;
        }

        player_left_right_motion(&mut p_data, &mut walker_d, &mut vel, input);

        if input.jump && p_data.can_jump {
            player_jump(&mut p_data, &mut walker_d, &mut vel);
        }
    }
}

