use crate::components::entity::{PlayerData, PlayerState};
use crate::components::WalkerData;

use PlayerState as P;

pub fn player_can_jump_delay_timer(p_data: &mut PlayerData, walker_d: &WalkerData, delta_time: f32) {
    if walker_d.grounded {
        p_data.can_jump = true;
        p_data.can_jump_timer = 0.0;
    }
    // if not on ground, wait for jump_delay seconds before can_jump is disabled
    else {
        p_data.can_jump_timer += delta_time;
        if p_data.can_jump_timer >= p_data.jump_delay {
            p_data.can_jump = false;
        }
    }
}

pub fn player_dodge_timer(p_data: &mut PlayerData, delta_time: f32) {
    if p_data.state != P::Dodging {
        return;
    }

    p_data.dodge_timer += delta_time;
    // make sure lerp timer is reset as you can dodge while lerping
    p_data.lerp_timer = 0.0;

    if p_data.dodge_timer >= p_data.dodge_duration {
        // transition to lerping
        p_data.state = P::Lerping;
        p_data.dodge_timer = 0.0;
        p_data.can_dodge = true;
    }
}

pub fn player_lerp_timer(p_data: &mut PlayerData, delta_time: f32) {
    if p_data.state != P::Lerping {
        return;
    }
    p_data.lerp_timer += delta_time;
    if p_data.lerp_timer >= p_data.lerp_duration {
        p_data.lerp_timer = 0.0;
        p_data.state = P::Rest;
    }
}
// void player_lerp_timer(Player* p, float dt) 
// {
//     p->lerp_timer += dt;
//     if (p->lerp_timer >= p->lerp_duration) {
//         p->lerp_timer = 0.0f;
//         // p->lerping = false;
//         CLEAR_FLAG(p->state_flags, PLAYER_LERPING);
//     }
// }
// 
// void player_dodge_timer(Player* p, float dt) 
// {
//     if (!HAS_FLAG(p->state_flags, PLAYER_DODGING)) {
//         return;
//     }
//     p->dodge_timer += dt;
// 
//     // prevent lerping to stack when dodge spamming
//     CLEAR_FLAG(p->state_flags, PLAYER_LERPING);
//     //p->lerping = false;
//     p->lerp_timer = 0.0f;
// 
//     if (p->dodge_timer >= p->dodge_duration) {
//         CLEAR_FLAG(p->state_flags, PLAYER_DODGING);
//         //p->dodging = false;
//         p->dodge_timer = 0.0f;
//         SET_FLAG(p->state_flags, PLAYER_CAN_DODGE);
//         // p->can_dodge = true;
// 
//         // activate lerping
//         // p->lerping = true;
//         SET_FLAG(p->state_flags, PLAYER_LERPING);
//     }
// }

