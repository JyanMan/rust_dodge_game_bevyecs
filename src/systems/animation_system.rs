use crate::components::animation_player::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;

pub fn animation_player_update() -> UpdateFn {
    Box::new(|ecs: &mut ECS, delta_time: f32| {
        for anim_player in ecs.query_comp::<&mut AnimationPlayer>() {
            anim_player.update_play(delta_time);
        }
    })
}
