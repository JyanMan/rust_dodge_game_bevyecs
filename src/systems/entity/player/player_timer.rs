use crate::components::entity::{PlayerData};
use crate::components::WalkerData;

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

pub fn player_can_dodge_timer(p_data: &mut PlayerData, delta_time: f32) {

}
