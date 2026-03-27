use bevy_ecs::prelude::*;

use crate::components::*;
use crate::components::states::*;

// pub fn update(mut query: Query<(&Velocity, &mut Sprite, &mut AnimationPlayer, &WalkerData)>) {
//     use WalkerState as S;
//     use WalkerAnim as A;
//     for (vel, mut sprite, mut anim_player, walker_d) in &mut query {
//         if vel.vec.x > 0.0 {
//             sprite.flip_x = false;
//         }
//         else if vel.vec.x < 0.0 {
//             sprite.flip_x = true;
//         }
        
//         match walker_d.state {
//             S::Idle => anim_player.play(A::Idle.usize()),
//             S::Running => anim_player.play(A::Run.usize()),
//             S::Aired => {
//                 if vel.vec.y < 0.0 {
//                     anim_player.play(A::Rise.usize())
//                 }
//                 else if vel.vec.y > 0.0 {
//                     anim_player.play(A::Fall.usize())
//                 }
//             },
//         }
//     }
// }

pub fn anim_state_handler(
    mut query: Query<(
        // &mut StateMachine<WalkerAnimState>,
        &mut AnimationPlayer,
        &mut Sprite,
        &StateMachine<MovementState>,
        &StateMachine<CombatState>,
        &WalkerData,
        &Velocity
    )>
) {
    use WalkerAnim as A;
    for (mut anim_player, mut sprite, movement_state, combat_state, walker_d, vel) in &mut query {
        let move_state = movement_state.curr_state();
        if vel.vec.x > 0.0 {
            sprite.flip_x = false;
        }
        else if vel.vec.x < 0.0 {
            sprite.flip_x = true;
        }
        match (move_state.clone(), combat_state.curr_state()) {
            (MovementState::Running | MovementState::Idle | MovementState::Dodging, CombatState::Idle) => {
                if walker_d.grounded {
                    if move_state == MovementState::Idle {
                        anim_player.play(A::Idle.usize());
                        // walker_state.set_state(WalkerAnimState::Idle); 
                    } 
                    else {
                        anim_player.play(A::Run.usize());
                        // walker_state.set_state(WalkerAnimState::Running); 
                    }
                }
                else if vel.vec.y > 0.0 {
                        anim_player.play(A::Fall.usize());
                        // walker_state.set_state(WalkerAnimState::Falling); 
                }
                else if vel.vec.y < 0.0 {
                    anim_player.play(A::Rise.usize());
                    // walker_state.set_state(WalkerAnimState::Rising); 
                }
            },
            (_, CombatState::Attacking) => {
                // walker_state.set_state(WalkerAnimState::Attacking);
                anim_player.play(A::Rise.usize());
            },
            _ => {}
        }
    }
    
}
