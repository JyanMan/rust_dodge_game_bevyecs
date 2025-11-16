use bevy_ecs::prelude::*;
// use std::collections::HashMap

use crate::resources::DeltaTime;
use crate::components::*;
use crate::systems::weapon::*;

#[allow(unused_mut)]
pub fn animation_player_update(
    mut query: Query<&mut AnimationPlayer>, 
    mut sprite_query: Query<&mut Sprite>,
    mut obb_query: Query<&mut OBB>,
    mut weapon_query: Query<&mut WeaponData>,
    delta_time: Res<DeltaTime>, 
    // mut commands: Commands,
    mut tmp_frames: Local<Vec<AnimFrame>>
) {
    /* bevy disallows the use of mutable world twice... so you separate the update_frame and timer
     * update of animation*/
    tmp_frames.clear();

    // let mut anim_frames: Vec<AnimFrame> = Vec::new();
    for mut anim_player in &mut query {
        if !anim_player.is_playing() { continue; }

        let frame_is_updated = anim_player.update_timer(delta_time.0);

        let curr_anim = anim_player.curr_anim();
        /* only store when frame is changed */
        if frame_is_updated {
            tmp_frames.push(curr_anim.curr_frame().clone());
        }
    }

    for anim_frame in tmp_frames.iter() {
        for anim_data in anim_frame.data.iter() {
            match anim_data {
                AnimData::SpriteFrame { value, target } => {
                    if let Ok(mut sprite) = sprite_query.get_mut(*target) {
                        sprite.frame = *value;
                    }
                },
                AnimData::OBBOffset { offset, target } => {
                    if let Ok(mut obb) = obb_query.get_mut(*target) {
                        obb.offset = *offset;
                    }
                },
                AnimData::OBBUpdate { target } => {
                    if let (Ok(mut obb), Ok(weapon_d)) =
                        (obb_query.get_mut(*target), weapon_query.get(*target))
                    {
                        steel_sword_per_frame_update(weapon_d, &mut obb);
                    }
                },
                AnimData::Debug{ msg } => {
                    // println!("ANIM_DEBUG: {}", msg);
                },
            }
        }
    }

    
    // for anim_data in anim_frame.data.iter() {
    //     match anim_data {
    //         AnimData::SpriteFrame { value, target } => {
    //             if let Ok(mut e) = world.get_entity_mut(*target) {
    //                 let mut sprite = e.get_mut::<Sprite>().expect("entity does not have sprite component");
    //                 sprite.frame = *value;
    //             }
    //         },
    //         AnimData::OBBOffset { offset, target } => {
    //             if let Ok(mut e) = world.get_entity_mut(*target) {
    //                 let mut obb = e.get_mut::<OBB>().expect("entity does not have sprite component");
    //                 obb.offset = *offset;
    //             }
    //             // obb.compute_vertices();
    //         },
    //         AnimData::OBBUpdate { target } => {
    //             steel_sword_per_frame_update(world, *target);
    //         },
    //         AnimData::Debug{ msg } => {
    //             println!("ANIM_DEBUG: {}", msg);
    //         },
    //     }
    // }

    /* call the callback of the new frame
     * anim_frame contains the necessary data for one frame update... no need for entire Animation*/
    
    // for anim_frame in tmp_frames.drain(..) {
    //     commands.queue(move |w: &mut World| {
    //         /* notice the lack of 'self'? using 'self' only angers the borrow checker, so you only need
    //          * the anim_frame*/
    //         Animation::update_frame(w, &anim_frame);
    //     });
    // }
}
