use crate::core::renderer::*;
use crate::components::animation::*;
use crate::components::animation_player::*;
use crate::components::walker_animation::*;
use crate::components::entity::PlayerData;
use crate::components::sprite::*;
use crate::ecs::ecs::*;

pub fn player_animation_init(ecs: &mut ECS, _renderer: &mut Renderer) {
    for (_e, sprite, anim_player, _p_tag) in 
        ecs.query_comp::<(&mut Sprite, &mut AnimationPlayer, &PlayerData)>() {

        let s_frame_ptr = &mut sprite.frame as *mut _;

        let mut idle_anim = Animation::new(4, 0.2);
        idle_anim.set_frame(0, AnimData::Integer { value: 0, target: s_frame_ptr});
        idle_anim.set_frame(1, AnimData::Integer { value: 1, target: s_frame_ptr});
        idle_anim.set_frame(2, AnimData::Integer { value: 2, target: s_frame_ptr});
        idle_anim.set_frame(3, AnimData::Integer { value: 3, target: s_frame_ptr});

        let mut run_anim = Animation::new(9, 0.1);
        run_anim.set_frame(0, AnimData::Integer { value: 4, target: s_frame_ptr});
        run_anim.set_frame(1, AnimData::Integer { value: 5, target: s_frame_ptr});
        run_anim.set_frame(2, AnimData::Integer { value: 6, target: s_frame_ptr});
        run_anim.set_frame(3, AnimData::Integer { value: 7, target: s_frame_ptr});
        run_anim.set_frame(4, AnimData::Integer { value: 8, target: s_frame_ptr});
        run_anim.set_frame(5, AnimData::Integer { value: 9, target: s_frame_ptr});
        run_anim.set_frame(6, AnimData::Integer { value: 10, target: s_frame_ptr});
        run_anim.set_frame(7, AnimData::Integer { value: 11, target: s_frame_ptr});
        run_anim.set_frame(8, AnimData::Integer { value: 12, target: s_frame_ptr});

        let mut rise_anim = Animation::new(1, 0.2);
        rise_anim.set_frame(0, AnimData::Integer { value: 13, target: s_frame_ptr});

        let mut fall_anim = Animation::new(1, 0.2);
        fall_anim.set_frame(0, AnimData::Integer { value: 15, target: s_frame_ptr});

        anim_player.add_anim(WalkerAnim::Idle.usize(), idle_anim);
        anim_player.add_anim(WalkerAnim::Run.usize(), run_anim);
        anim_player.add_anim(WalkerAnim::Rise.usize(), rise_anim);
        anim_player.add_anim(WalkerAnim::Fall.usize(), fall_anim);
        // anim_player.play(PAnims::Run.usize());
    }
}

// pub fn player_animation_update(ecs: &mut ECS, _delta_time: f32) {
//     for (_e, p_input, p_data, sprite, anim_player, vel) in 
//         ecs.query_comp::<(&PlayerInput, &WalkerData, &mut Sprite, &mut AnimationPlayer, &Velocity)>() {
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
