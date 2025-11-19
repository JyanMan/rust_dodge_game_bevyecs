use crate::components::entity::{PlayerData, PlayerInput, PlayerState};
use crate::components::*;
use crate::config::*;
use crate::resources::MouseInput;

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
        vel.vec.x += run_dir * walker_d.accel;
        if vel.vec.x.abs() >= walker_d.run_speed {
            vel.vec.x = walker_d.run_speed.copysign(run_dir);
        }
    }
    else {
        walker_d.state = WalkerState::Idle;
        if vel.vec.x.abs() > walker_d.accel {
            vel.vec.x -= walker_d.accel.copysign(vel.vec.x);
        }
        else {
            vel.vec.x = 0.0;
        }
    }
}

pub fn player_jump(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity
) {
    vel.vec.y = -walker_d.jump_force;
    p_data.state = P::Rest;
    //p_data.state.clear(P::Jumping);
    p_data.can_jump = false;
}

pub fn get_dodge_dir(mouse_pos: Vector2, p_data: &PlayerData) -> Vector2 {
    let mouse_input = MouseInput { pos: mouse_pos };

    // get mouse direction and distance from player center
    // let mut mouse_delta = mouse_pos - screen_center;
    let mut mouse_delta = mouse_input.dist_to_center();
    let mouse_dir = mouse_delta.normalize();

    // limit dodge dir from min to max
    let mouse_delta_len = mouse_delta.len();
    if mouse_delta_len <= p_data.dodge_min {
        mouse_delta = mouse_dir * p_data.dodge_min;
    }
    else if  mouse_delta_len >= p_data.dodge_max {
        mouse_delta = mouse_dir * p_data.dodge_max;
    }
    mouse_delta
}

pub fn player_dodging(dodge_dir: Vector2, p_data: &mut PlayerData, vel: &mut Velocity, health: &mut Health) {
    vel.vec = Vector2::zero();
    vel.vec = vel.vec + (dodge_dir * p_data.dodge_speed);
    health.set_immune();
}

pub fn player_lerping(vel: &mut Velocity) {
    vel.vec = vel.vec.normalize() * 25.0 
}

pub fn player_dodge(
    // ecs: &ECS,
    p_data: &mut PlayerData, 
    // vel: &mut Velocity,
) {
    if p_data.state == P::Dodging {
        // let dodge_dir = get_dodge_dir(ecs, p_data);
        // player_dodging(dodge_dir, p_data, vel);
        return;
    }
    p_data.can_dodge = false;
    p_data.dodge_timer = 0.0;
    p_data.state = P::Dodging;
}

