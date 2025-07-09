use crate::components::entity::{PlayerData, PlayerInput, PlayerState};
use crate::components::{
    WalkerData,
    WalkerState,
    Velocity,
    Vector2,
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
        vel.vec.x += run_dir * walker_d.accel;
        if vel.vec.x.abs() >= walker_d.run_speed {
            vel.vec.x = walker_d.run_speed.copysign(run_dir);
        }
    }
    else {
        walker_d.state = WalkerState::Idle;
        if vel.vec.x.abs() > 0.001 {
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

// fn get_dodge_dir( pos: &Position) {
// 
// }

fn player_dodging(pos: &Position, p_data: &mut PlayerData, vel: &mut Velocity) {

}

pub fn player_dodge(
    p_data: &mut PlayerData, 
    walker_d: &mut WalkerData, 
    vel: &mut Velocity,
    pos: &Position,
) {
    if p_data.state == P::Dodging {
        player_dodging(pos, p_data, vel);
        return;
    }
    p_data.can_dodge = false;
    p_data.state = P::Dodging;
}
//void player_dodge(Player* p) 
//{
//    if (HAS_FLAG(p->state_flags, PLAYER_DODGING)) {
//        return;
//    }
//    // p->can_dodge = false;
//    CLEAR_FLAG(p->state_flags, PLAYER_CAN_DODGE);
//    // p->dodging = true;
//    SET_FLAG(p->state_flags, PLAYER_DODGING);
//    printf("dodged\n");
//}
//
//void player_dodging(Player* p, float dt) 
//{
//    Vector2 dodge_dir = player_dodge_dir(p);
//    p->entity->velocity = (Vector2) {0, 0};
//    p->entity->velocity = vector2_sum(p->entity->velocity, vector2_scale(dodge_dir, p->dodge_speed));
//}
//
//void player_lerping(Player* p, float dt)
//{
//    p->entity->velocity = vector2_scale(normalize_vector(&p->entity->velocity), 25.0f);
//}
//
//Vector2 player_dodge_dir(Player* p) 
//{
//    // get mouse pos
//    int x, y;
//    SDL_GetMouseState(&x, &y);
//    Vector2 mouse_pos = {x, y};
//
//    // get player center
//    Vector2 screen_center = {(float)SCREEN_WIDTH / 2, (float)SCREEN_HEIGHT / 2};
//    // screen_center = vector2_sum(screen_center, (Vector2) {
//    //     (float)roundf(p->entity.sprite->width) / 2, (float)roundf(p->entity.sprite->height) / 2
//    // }); //this center is inaccurate
//
//    // get mouse direction and distance from player center
//    Vector2 mouse_delta = vector2_sub(mouse_pos, screen_center);
//    Vector2 mouse_dir = normalize_vector(&mouse_delta);
//
//    // limit dodge dir from min to max
//    if (vector_len(&mouse_delta) <= p->dodge_min) {
//        printf("low\n");
//        mouse_delta = vector2_scale(mouse_dir, p->dodge_min);
//    }
//    else if (vector_len(&mouse_delta) >= p->dodge_max) {
//        normalize_vector(&mouse_delta);
//        mouse_delta = vector2_scale(mouse_dir, p->dodge_max);
//    }
//    return mouse_delta; 
//}

