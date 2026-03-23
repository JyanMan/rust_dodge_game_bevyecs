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
use crate::components;
use crate::components::states::*;
use crate::resources::*;
use crate::components::entity::*;

pub fn timers_update(
    mut query: Query<(&mut PlayerData, &mut WalkerData,
    &mut StateMachine<components::states::MovementState>)>,
    delta_time: Res<DeltaTime>
) {
    use player_timer;
    // use super::player_timer::*;
    for (mut p_data, walker_d, mut state_m) in &mut query {
        player_timer::can_jump_delay_timer(&mut p_data, &walker_d, &mut state_m, delta_time.0);
        match state_m.curr_state() {
            components::states::MovementState::Dodging => {
                player_timer::dodge_timer(&mut p_data, &mut state_m, delta_time.0);
            },
            // StateId::DodgeAttacking => {
            // components::states::MovementState::Dodging => {
            //     player_timer::dodge_timer(&mut p_data, &mut state_m, delta_time.0);
            // }
            components::states::MovementState::DodgeLerping => {
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
        &KnockbackTrigger,
        &mut StateMachine<components::states::MovementState>,
        &mut StateMachine<components::states::CombatState>
    )>, 
) {
    for (p_data, input, combat, knock, mut movement_state, mut combat_state) in &mut query {
         
        if input.dodge && p_data.can_dodge {
            movement_state.set_state(MovementState::StartDodge);
            return;
        }
        if input.right || input.left {
            movement_state.set_state(MovementState::Running);
        }
        if knock.knocked && movement_state.curr_state() != MovementState::Dodging {
            combat_state.set_state(CombatState::Knocked);
        }
        else if combat.attacking {
            combat_state.set_state(CombatState::Attacking);
        }
        else {
            combat_state.set_state(CombatState::StopAttacking);
            combat_state.set_state(CombatState::Idle);
        }
        movement_state.set_state(MovementState::Idle);

    }
}

pub fn state_handler(
    mut query: Query<(
        &mut PlayerData, &mut WalkerData, &mut Velocity, &mut Health, &PlayerInput,
        &Combat,
        &mut StateMachine<MovementState>,
        &mut StateMachine<CombatState>
    )>, 
    mouse_input: Res<MouseInput>
) {
    use player_movement;

    let mouse_pos = mouse_input.pos;

    for (mut p_data, mut walker_d, mut vel, mut health, input, combat, mut movement_state, mut combat_state) in &mut query {

        match movement_state.curr_state() {
            MovementState::StartDodge => {
                player_movement::dodge(&mut p_data);
                movement_state.set_state(MovementState::Dodging);
            }
            MovementState::Dodging => {
                let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
                player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
                // p_data.state = P::Dodging;
            },
            MovementState::DodgeLerping => {
                player_movement::lerping(&mut vel);

                // only allow dodge once button was let go
                if !input.dodge {
                    p_data.can_dodge = true;
                }
            },
            MovementState::Running => {
                if combat_state.curr_state() == CombatState::Idle {
                    player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                    if input.jump && p_data.can_jump {
                        player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                    }
                }
            }
            MovementState::Idle => {
                // only allow dodge once button was let go
                if combat_state.curr_state() == CombatState::Idle {
                    if !input.dodge {
                        p_data.can_dodge = true;
                    }
                    player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                    if input.jump && p_data.can_jump {
                        player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                    }
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
