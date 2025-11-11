use bevy_ecs::prelude::*;

use crate::components::animation_player::*;
use crate::resources::DeltaTime;
use crate::components::animation::*;

pub fn animation_player_update(
    mut query: Query<&mut AnimationPlayer>, 
    delta_time: Res<DeltaTime>, 
    mut commands: Commands,
    mut tmp_frames: Local<Vec<AnimFrame>>
) {
    /* bevy disallows the use of mutable world twice... so you separate the update_frame and timer
     * update of animation*/
    tmp_frames.clear();

    // let mut anim_frames: Vec<AnimFrame> = Vec::new();
    for mut anim_player in &mut query {
        if !anim_player.is_playing() { continue; }

        let curr_anim = anim_player.curr_anim();
        /* only call update frame when timer is reset */
        if curr_anim.play_timer() == 0.0 {
            tmp_frames.push(curr_anim.curr_frame().clone());
        }
        anim_player.update_timer(delta_time.0);
    }
    /* call the callback of the new frame
     * anim_frame contains the necessary data for one frame update... no need for entire Animation*/
    for anim_frame in tmp_frames.drain(..) {
        commands.queue(move |w: &mut World| {
            /* notice the lack of 'self'? using 'self' only angers the borrow checker, so you only need
             * the anim_frame*/
            Animation::update_frame(w, &anim_frame);
        });
    }
}
