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

use PlayerState as P;

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

pub fn movement_update(
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
        //  if input.dodge && p_data.can_dodge {
        //     player_movement::dodge(&mut p_data);
        // }

        // // only allow dodge again if dodge button is let go
        // // otherwise, player can fly like superman
        // if p_data.state != P::Dodging && !input.dodge {
        //     p_data.can_dodge = true; 
        // }
        // if p_data.state == P::Dodging {
        //     let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
        //     player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
        //     return;
        // } 
        // if p_data.state == P::Lerping {
        //     player_movement::lerping(&mut vel);
        //     return;
        // }
        // if combat.attacking {
        //     // vel.vec = vel.vec * 0.5;
        //     // steel_sword_movement_effect(&mut vel, mouse_input.clone());
        //     return;
        // }

        // player_movement::left_right_motion(&mut walker_d, &mut vel, input);

        // if input.jump && p_data.can_jump {
        //     player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
        // }
         
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
            // println!("HEY iM STOPPING AIN'T I??");
            state_m.set_state(StateId::StopAttacking);
        }
        state_m.set_state(StateId::Idle);

    }
}

pub fn state_machine_handler(
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
                println!("START DODGE");
                player_movement::dodge(&mut p_data);
                state_m.set_state(StateId::Dodging);
                p_data.state = P::Rest;
            }
            StateId::Dodging => {
                println!("DODGING");
                let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
                player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
                p_data.state = P::Dodging;
                // p_data.state = P::Dodging;
            },
            StateId::DodgeLerping => {
                println!("LERPING");
                player_movement::lerping(&mut vel);
                p_data.state = P::Lerping;

                // only allow dodge once button was let go
                if !input.dodge {
                    p_data.can_dodge = true;
                }
            },
            StateId::DodgeAttacking => {
                println!("DODGING");
                let dodge_dir = player_movement::get_dodge_dir(mouse_pos, &p_data);
                player_movement::dodging(dodge_dir, &mut p_data, &mut vel, &mut health);
                p_data.state = P::Dodging;
            }
            StateId::Running => {
                println!("RUNNIN");
                player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                if input.jump && p_data.can_jump {
                    player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                }
            }
            StateId::Idle => {
                println!("IDLE");
                // only allow dodge once button was let go
                if !input.dodge {
                    p_data.can_dodge = true;
                }
                player_movement::left_right_motion(&mut walker_d, &mut vel, input);
                if input.jump && p_data.can_jump {
                    player_movement::jump(&mut p_data, &mut walker_d, &mut vel);
                }
            }
            StateId::Attacking => {
                println!("ATTACKING");
            }
            StateId::StopAttacking => {
                println!("asdfasdfasdf");
            }
            _ => {
                println!("WTF ARE YOU??");
            }
        }

    }
}

#[allow(dead_code)]
pub fn test_overlap(
    mut query: Query<(&PlayerTag, &EntityOverlappingOBBs)>, 
) {
    for (_e, e_over_obbs) in &mut query {
        if e_over_obbs.0.len() != 0 {
            println!("player is overlapping");
        }
    }
}
