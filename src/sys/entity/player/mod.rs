pub mod player_animation_init_system;
pub mod player_spawn;
pub mod player_input;
mod player_movement;
mod player_timer;
mod states;

// pub use player_spawn::*;
pub use player_input::*;
// pub use player_system::*;
pub use player_animation_init_system::*;
pub use player_spawn::spawn;

use bevy_ecs::prelude::*;
use crate::components::entity::WalkerData;
use crate::components::*;
use crate::resources::*;
use crate::components::entity::*;

pub fn timers_update(
    mut query: Query<(&mut PlayerData, &mut WalkerData, &mut StateMachine)>,
    delta_time: Res<DeltaTime>
) {
    use player_timer;
    // use super::player_timer::*;
    for (mut p_data, walker_d, mut state_m) in &mut query {
        player_timer::can_jump_delay_timer(&mut p_data, &walker_d, &mut state_m, delta_time.0);
        match state_m.curr_state() {
            StateId::Dodging => {
                player_timer::dodge_timer(&mut p_data, &mut state_m, delta_time.0);
            },
            StateId::DodgeAttacking => {
                player_timer::dodge_timer(&mut p_data, &mut state_m, delta_time.0);
            }
            StateId::DodgeLerping => {
                player_timer::lerp_timer(&mut p_data, &mut state_m, delta_time.0);
            }
            _ => {}
            
        }
    } 
}

pub fn state_update(
    mut query: Query<(
        &PlayerData, &PlayerInput,
        &Combat,
        &mut StateMachine
    )>, 
) {
    for (p_data, input, combat, mut state_m) in &mut query {
         
        if input.dodge && p_data.can_dodge {
            state_m.set_state(StateId::StartDodge);
            return;
        }
        if input.right || input.left {
            state_m.set_state(StateId::Running);
        }
        if combat.attacking {
            state_m.set_state(StateId::Attacking);
        }
        else {
            state_m.set_state(StateId::StopAttacking);
        }
        state_m.set_state(StateId::Idle);

    }
}

pub fn state_handler(
    mut query: Query<(
        &mut PlayerData, &mut WalkerData, &mut Velocity, &mut Health, &PlayerInput,
        &Combat,
        &mut StateMachine
    )>, 
    mouse_input: Res<MouseInput>
) {
    use player_movement;

    let mouse_pos = mouse_input.pos;

    for (mut p_data, mut walker_d, mut vel, mut health, input, combat, mut state_m) in &mut query {

        match state_m.curr_state() {
            StateId::StartDodge => {
                player_movement::dodge(&mut p_data);
                state_m.set_state(StateId::Dodging);
            }
            StateId::Dodging => {
                let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
                player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
                // p_data.state = P::Dodging;
            },
            StateId::DodgeLerping => {
                player_movement::lerping(&mut vel);

                // only allow dodge once button was let go
                if !input.dodge {
                    p_data.can_dodge = true;
                }
            },
            StateId::DodgeAttacking => {
                let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
                player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
            }
            StateId::Running => {
                player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                if input.jump && p_data.can_jump {
                    player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                }
            }
            StateId::Idle => {
                // only allow dodge once button was let go
                if !input.dodge {
                    p_data.can_dodge = true;
                }
                player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                if input.jump && p_data.can_jump {
                    player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                }
            }
            _ => {}
        }

    }
}

#[allow(dead_code)]
pub fn test_overlap(
    mut query: Query<(&PlayerTag, &EntityOverlappingOBBs)>, 
) {
    for (_e, e_over_obbs) in &mut query {
        if !e_over_obbs.0.is_empty() {
            println!("player is overlapping");
        }
    }
}
