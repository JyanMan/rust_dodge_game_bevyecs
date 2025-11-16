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
) {

    // let mut anim_frames: Vec<AnimFrame> = Vec::new();
    for mut anim_player in &mut query {
        if !anim_player.is_playing() { continue; }

        let frame_is_updated = anim_player.update_timer(delta_time.0);

        let curr_anim = anim_player.curr_anim();
        /* only store when frame is changed */
        if !frame_is_updated {
            continue;
        }
        for anim_data in curr_anim.curr_frame().data.iter() {
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
                    println!("ANIM_DEBUG: {}", msg);
                },
            }
        }
    }
}
