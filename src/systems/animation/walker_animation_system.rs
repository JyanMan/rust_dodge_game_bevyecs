use bevy_ecs::prelude::*;

use crate::components::animation_player::*;
use crate::components::sprite::*;
//use crate::components::state_machine::*;
use crate::components::entity::{ WalkerAnim, WalkerState, WalkerData };
use crate::components::velocity::*;

pub fn walker_animation_update(mut query: Query<(&Velocity, &mut Sprite, &mut AnimationPlayer, &WalkerData)>) {
    use WalkerState as S;
    use WalkerAnim as A;
    for (vel, mut sprite, mut anim_player, walker_d) in &mut query {
        if vel.vec.x > 0.0 {
            sprite.flip_x = false;
        }
        else if vel.vec.x < 0.0 {
            sprite.flip_x = true;
        }
        
        match walker_d.state {
            S::Idle => anim_player.play(A::Idle.usize()),
            S::Running => anim_player.play(A::Run.usize()),
            S::Aired => {
                if vel.vec.y < 0.0 {
                    anim_player.play(A::Rise.usize())
                }
                else if vel.vec.y > 0.0 {
                    anim_player.play(A::Fall.usize())
                }
            },
        }
    }
}
