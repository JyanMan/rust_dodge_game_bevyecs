use bevy_ecs::prelude::*;

use crate::components::animation_player::*;
use crate::resources::DeltaTime;
use crate::components::animation::*;

pub fn animation_player_update(mut query: Query<(Entity, &mut AnimationPlayer)>, delta_time: Res<DeltaTime>, mut commands: Commands) {
    /* bevy disallows the use of mutable world twice... so you separate the update_frame and timer
     * update of animation*/
    let mut anim_frames: Vec<AnimFrame> = Vec::new();
    for (_e, mut anim_player) in &mut query {
        let curr_anim = anim_player.curr_anim();
        /* only call update frame when timer is reset */
        if curr_anim.play_timer() == 0.0 {
            anim_frames.push(curr_anim.curr_frame().clone());
        }
        anim_player.update_timer(delta_time.0);
    }
    /* call the callback of the new frame
     * anim_frame contains the necessary data for one frame update... no need for entire Animation*/
    commands.queue(move |w: &mut World| {
        for anim_frame in anim_frames {
            /* notice the lack of 'self'? using 'self' only angers the borrow checker, so you only need
             * the anim_frame*/
            Animation::update_frame(w, &anim_frame);
        }
    });
}
