pub mod walker;

use bevy_ecs::prelude::*;

use crate::resources::DeltaTime;
use crate::components::*;
use crate::sys;

// WARNING: for now this performs .get all frames per frame
#[allow(unused_mut)]
pub fn update_all(
    mut query: Query<&mut AnimationPlayer>, 
    mut sprite_query: Query<&mut Sprite>,
    // mut trans_query: Query<&mut Transform>,
    mut local_trans_query: Query<&mut LocalTransform>,
    delta_time: Res<DeltaTime>, 
) {

    for mut anim_player in &mut query {
        if !anim_player.is_playing() { continue; }

        anim_player.update_timer(delta_time.0);

        let curr_anim = anim_player.curr_anim();
        let anim_frames = curr_anim.anim_frames();
        let curr_frame_idx = curr_anim.curr_frame_idx();
        let curr_frames = anim_frames.get_frames(curr_frame_idx);
        // let curr_frame = anim_frames.get(curr_anim.curr_frame_idx()).expect("somehow out of bounds frame index");

        for (x, anim_data) in curr_frames.iter().enumerate() {

            let anim_data =
                if let Some(anim_data) = anim_data { anim_data }
                else {continue;};

            let mut y = curr_frame_idx+1;

            // this reads the next row from the same column checks
            // if none then read the next
            let (next_anim_data, num_frame_ahead) = loop {
                if y >= curr_anim.frame_num {
                    break (None, y);
                }
                let frames = anim_frames.get_frames(y);
                if let Some(res) = &frames[x] {
                    break (Some(res), y);
                }
                y += 1;
            };

            // let prev_anim_data =
            //     if let Some(prev_anim_data) = prev_anim_data { prev_anim_data }
            //     else {continue;};
            

            let delta_idx = num_frame_ahead - curr_frame_idx;
            let elapsed = (delta_idx as f32 * curr_anim.s_per_frame)
                + anim_player.elapsed_between_frames;
            let ratio = elapsed / ((delta_idx + 1) as f32 * curr_anim.s_per_frame);

            match anim_data {
                // TODO: you can optimize querying for sprite just once for angle and frame change
                AnimData::SpriteFrame { value, target } => {
                    if let Ok(mut sprite) = sprite_query.get_mut(*target) {
                        sprite.frame = *value as u32;
                    }
                },
                AnimData::SpriteAngle { value, target } => {
                    if let Ok(mut sprite) = sprite_query.get_mut(*target) {
                        #[allow(clippy::collapsible_if)]
                        if let Some(next_anim_data) = next_anim_data {
                            if let AnimData::SpriteAngle{value: next_val, target:_target} = next_anim_data {
                                sprite.angle = *value + (next_val - *value) * ratio as f64;
                            }
                            else {
                                println!("AYOW??");
                                sprite.angle = *value;
                            }
                        }
                    }
                },
                AnimData::TransformLocal{ value, target } => {
                    if let Ok(mut trans) =
                        local_trans_query.get_mut(*target)
                    {
                        #[allow(clippy::collapsible_if)]
                        if let Some(next_anim_data) = next_anim_data {
                            if let AnimData::TransformLocal{value: next_val, target:_target} = next_anim_data {
                                trans.pos = *value + (next_val - *value) * ratio;
                            }
                            else {
                                trans.pos = *value;
                            }
                        }
                    }
                },
                AnimData::Debug{ msg } => {
                    println!("ANIM_DEBUG: {}", msg);
                },
            }
        }
    }
}
