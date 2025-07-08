use crate::components::animation_player::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;

pub fn animation_player_update(ecs: &mut ECS, delta_time: f32) {
    for (_e, anim_player) in ecs.query_comp::<&mut AnimationPlayer>() {
        anim_player.update_play(delta_time);
    }
}
