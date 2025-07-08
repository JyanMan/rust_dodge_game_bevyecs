use crate::components::entity::PlayerData;
use crate::components::{
    WalkerData,
    WalkerState,
    Velocity
};

pub fn player_left_right_motion(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity
) {
    let mut run_dir: f32 = 0.0;
    // let mut vert_dir: f32 = 0.0;
    if p_data.run_dir == 1 {
        run_dir += 1.0;
    }
    if p_data.run_dir == -1 {
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

pub fn player_dodge(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity
) {

}
