use crate::components::entity::{PlayerData, PlayerInput, PlayerState};
use crate::components::{
    WalkerData,
    WalkerState,
    Velocity,
    Position,
};

use PlayerState as P;

pub fn player_left_right_motion(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity,
    input: &PlayerInput
) {
    let mut run_dir: f32 = 0.0;
    // let mut vert_dir: f32 = 0.0;
    if input.right {
        run_dir += 1.0;
    }
    if input.left {
        run_dir -= 1.0;
    }
    // let run_speed = walker_d.run_speed;
    if run_dir != 0.0 {
        walker_d.state = WalkerState::Running;
        //state_m.set_state(State::Running);
        vel.x += run_dir * walker_d.accel;
        if vel.x.abs() >= walker_d.run_speed {
            vel.x = walker_d.run_speed.copysign(run_dir);
        }
    }
    else {
        walker_d.state = WalkerState::Idle;
        if vel.x.abs() > 0.001 {
            vel.x -= walker_d.accel.copysign(vel.x);
        }
        else {
            vel.x = 0.0;
        }
    }
}

pub fn player_jump(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity
) {
    vel.y = -walker_d.jump_force;
    p_data.state = P::Rest;
    //p_data.state.clear(P::Jumping);
    p_data.can_jump = false;
}

fn get_dodge_dir(
    pos: &Position
) {

}

pub fn player_dodge(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity,
    pos: &Position,
) {
    p_data.can_dodge = false;
}
