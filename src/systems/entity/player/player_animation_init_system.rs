use bevy_ecs::prelude::*;

use crate::components::animation::*;
use crate::components::animation_player::*;
use crate::components::entity::*;


pub fn player_animation_init(anim_player: &mut AnimationPlayer, player_e: Entity) {

    let idle_anim = Animation::new(0.2, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 0, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 1, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 2, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 3, target: player_e} ] ),
    ]);

    let run_anim = Animation::new(0.1, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 4, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 5, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 6, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 7, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 8, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 9, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 10, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 11, target: player_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 12, target: player_e} ] ),
    ]);

    let rise_anim = Animation::new(0.2, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 13, target: player_e} ] ),
    ]);

    let fall_anim = Animation::new(0.2, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 15, target: player_e} ] ),
    ]);

    anim_player.add_anim(WalkerAnim::Idle.usize(), idle_anim);
    anim_player.add_anim(WalkerAnim::Run.usize(), run_anim);
    anim_player.add_anim(WalkerAnim::Rise.usize(), rise_anim);
    anim_player.add_anim(WalkerAnim::Fall.usize(), fall_anim);
}
