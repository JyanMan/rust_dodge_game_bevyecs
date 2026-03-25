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

// pub fn movement_state_update(
//     mut query: Query<(
//         &PlayerData, &PlayerInput,
//         &mut StateMachine<components::states::MovementState>,
//     )>, 
// ) {
//     for (p_data, input, mut movement_state) in &mut query {
         
//         movement_state.set_state(MovementState::Idle);
//     }
// }


// pub fn from_player_input_update(
//     mut query: Query<(&PlayerInput, &mut Combat), (With<HeldItem>, Without<HeldBy>)>, 
//     mouse_input: Res<MouseInput>,
// ) {
//     for (input, mut combat) in &mut query{
//         if input.attack {
//             let attack_dir = mouse_input.dir_from_center();
//             combat.attack(attack_dir);
//         }
//         else { combat.not_attack() }
//     }
// }



pub fn state_handler(
    mut query: Query<(
        &mut PlayerData, &mut WalkerData, &mut Velocity, &mut Health, &PlayerInput,
        &mut Combat,
        &mut StateMachine<MovementState>,
        &mut StateMachine<CombatState>,
        &mut GravityAffected,
    )>, 
    mouse_input: Res<MouseInput>
) {
    use player_movement;

    let mouse_pos = mouse_input.pos;

    for (mut p_data, mut walker_d, mut vel, mut health, input, mut combat, mut movement_state, mut combat_state, mut gravity) in &mut query {

        match movement_state.curr_state() {
            MovementState::StartDodge => {
                player_movement::dodge(&mut p_data);
                movement_state.set_state(MovementState::Dodging);
                gravity.0 = false;
            }
            MovementState::Dodging => {
                let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
                player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
            },
            MovementState::DodgeLerping => {
                health.set_immune();
                player_movement::lerping(&mut vel);
            },
            MovementState::DodgeEnd => {
                movement_state.set_state(MovementState::Idle);
                gravity.0 = true;
            }
            MovementState::StartJump => {
                player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                movement_state.set_state(MovementState::Idle);
            }
            MovementState::Running => {
                if combat_state.curr_state() != CombatState::Attacking{
                    player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                }
            }
            MovementState::Idle => {
                if combat_state.curr_state() != CombatState::Attacking {
                    player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                }
            }
            _ => {}
        }

        if !input.dodge {
            p_data.can_dodge = true;
        }
        if input.dodge && p_data.can_dodge {
            movement_state.set_state(MovementState::StartDodge);
        }
        if input.right || input.left {
            movement_state.set_state(MovementState::Running{});
        }
        else {
            movement_state.set_state(MovementState::Idle);
        }
        if input.jump && p_data.can_jump {
            movement_state.set_state(MovementState::StartJump);
        }

        if input.use_item && combat.can_attack {
            let attack_dir = mouse_input.dir_from_center();
            combat_state.set_state(CombatState::StartAttack);
            combat.attack_dir = attack_dir;
            combat.can_attack = false;
        }
        if !input.use_item {
            combat.can_attack = true;
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
