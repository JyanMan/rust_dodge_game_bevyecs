use bevy_ecs::prelude::*;

use crate::core::renderer::*;
use crate::components::animation::*;
use crate::components::animation_player::*;
use crate::components::entity::*;
use crate::components::Velocity;
use crate::components::sprite::*;

// use WalkerAnim as PAnims;

pub fn player_animation_init(anim_player: &mut AnimationPlayer, player_e: Entity) {
    // for (_e, sprite, anim_player, _p_tag) in 
        // ecs.query_comp::<(&mut Sprite, &mut AnimationPlayer, &PlayerData)>() {

        // let s_frame_ptr = &mut sprite.frame as *mut _;

    let mut idle_anim = Animation::new(4, 0.2);
    idle_anim.set_frame(0, AnimData::SpriteFrame { value: 0, target: player_e});
    idle_anim.set_frame(1, AnimData::SpriteFrame { value: 1, target: player_e});
    idle_anim.set_frame(2, AnimData::SpriteFrame { value: 2, target: player_e});
    idle_anim.set_frame(3, AnimData::SpriteFrame { value: 3, target: player_e});

    let mut run_anim = Animation::new(9, 0.1);
    run_anim.set_frame(0, AnimData::SpriteFrame { value: 4, target: player_e});
    run_anim.set_frame(1, AnimData::SpriteFrame { value: 5, target: player_e});
    run_anim.set_frame(2, AnimData::SpriteFrame { value: 6, target: player_e});
    run_anim.set_frame(3, AnimData::SpriteFrame { value: 7, target: player_e});
    run_anim.set_frame(4, AnimData::SpriteFrame { value: 8, target: player_e});
    run_anim.set_frame(5, AnimData::SpriteFrame { value: 9, target: player_e});
    run_anim.set_frame(6, AnimData::SpriteFrame { value: 10, target: player_e});
    run_anim.set_frame(7, AnimData::SpriteFrame { value: 11, target: player_e});
    run_anim.set_frame(8, AnimData::SpriteFrame { value: 12, target: player_e});

    let mut rise_anim = Animation::new(1, 0.2);
    rise_anim.set_frame(0, AnimData::SpriteFrame { value: 13, target: player_e});

    let mut fall_anim = Animation::new(1, 0.2);
    fall_anim.set_frame(0, AnimData::SpriteFrame { value: 15, target: player_e});

    anim_player.add_anim(WalkerAnim::Idle.usize(), idle_anim);
    anim_player.add_anim(WalkerAnim::Run.usize(), run_anim);
    anim_player.add_anim(WalkerAnim::Rise.usize(), rise_anim);
    anim_player.add_anim(WalkerAnim::Fall.usize(), fall_anim);
        // anim_player.play(PAnims::Run.usize());
    // }
}

// pub fn player_animation_update(mut query: Query<(&PlayerInput, &WalkerData, &mut Sprite, &mut AnimationPlayer, &Velocity)>) {
//     for (p_input, p_data, sprite, anim_player, vel) in &mut query {
// 
//         let mut anim_index = PAnims::Idle.usize();
// 
//         if p_input.run_dir > 0 {
//             sprite.flip_x = false;
//             anim_index = PAnims::Run.usize();
//         }
//         else if p_input.run_dir < 0 {
//             sprite.flip_x = true;
//             anim_index = PAnims::Run.usize();
//         }
//         
//         if !p_data.grounded {
//             if vel.y <= 0.0 {
//                 anim_index = PAnims::Rise.usize();
//             }
//             else if vel.y > 0.0 {
//                 anim_index = PAnims::Fall.usize();
//             }
//         }
// 
//         anim_player.play(anim_index);
//     }
// }
