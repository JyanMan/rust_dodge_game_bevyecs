use bevy_ecs::prelude::*;

use crate::components::animation_player::*;
use crate::resources::DeltaTimeRes;
use crate::components::animation::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;

pub fn animation_player_update(mut query: Query<(Entity, &mut AnimationPlayer)>, dt_res: Res<DeltaTimeRes>, mut commands: Commands) {
    // use a vector to store a copy of the animationplayer or maybe animations to animation... then
    let mut anim_frames: Vec<AnimFrame> = Vec::new();
    let delta_time = dt_res.delta_time;
    for (_e, mut anim_player) in &mut query {
        let curr_anim = anim_player.curr_anim();
        if curr_anim.play_timer() == 0.0 {
            anim_frames.push(curr_anim .curr_frame() .clone());
        }
        anim_player.update_play(delta_time);
    }
    // manually call the animations in the vector here to pass in the mutable world
    commands.queue(move |w: &mut World| {
        for anim_frame in anim_frames {
            Animation::update_frame(w, &anim_frame);
        }
    });
}
