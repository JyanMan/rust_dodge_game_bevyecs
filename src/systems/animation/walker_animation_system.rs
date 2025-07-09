use crate::components::animation_player::*;
use crate::components::sprite::*;
//use crate::components::state_machine::*;
use crate::components::entity::{ WalkerAnim, WalkerState, WalkerData };
use crate::components::velocity::*;
use crate::ecs::ecs::*;

pub fn walker_animation_update(ecs: &mut ECS, _delta_time: f32) {
    use WalkerState as S;
    use WalkerAnim as A;
    for (_e, vel, sprite, anim_player, walker_d) 
        in ecs.query_comp::<(&Velocity, &mut Sprite, &mut AnimationPlayer, &WalkerData)>() 
    {
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
                if vel.vec.y <= 0.0 {
                    anim_player.play(A::Rise.usize())
                }
                else {
                    anim_player.play(A::Fall.usize())
                }
            },
        }
    }
}
