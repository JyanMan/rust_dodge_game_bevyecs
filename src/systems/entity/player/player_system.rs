use sdl2::keyboard::*; 
use crate::core::renderer::*;
use crate::components::animation_player::*;
use crate::components::sprite::*;
use crate::components::walker_data::*;
use crate::components::walker_state::*;
use crate::components::walker_animation::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::components::area::*;
use crate::ecs::ecs::*;
use crate::managers::asset_manager::*;
use crate::components::entity::*;

use PState as P;

pub fn player_init(ecs: &mut ECS, renderer: &mut Renderer) {

    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Position::new(12.0, 12.0);

    ecs.spawn::<(Sprite, Position, Velocity, Area, PlayerData, WalkerData, AnimationPlayer)>((
        sprite,
        Position { x: 10.0, y: -1000.0 },
        Velocity { x: 0.0, y: 0.0 },
        area,
        PlayerData::default(),
        WalkerData {
            run_speed: 200.0,
            accel: 50.0,
            jump_force: 300.0,
            grounded: false,
            state: WalkerState::default()
        },
        AnimationPlayer::new(WalkerAnim::COUNT),
        // StateMachine::default(),
    ));
}

pub fn player_update(ecs: &mut ECS, delta_time: f32) {
    for (_e, p_data, walker_d, vel) in 
        ecs.query_comp::<(&mut PlayerData, &WalkerData, &mut Velocity)>() {

        if walker_d.grounded {
            p_data.state.set(P::CanJump);
            p_data.can_jump_timer = 0.0;
        }
        // if not on ground, wait for jump_delay seconds before can_jump is disabled
        else {
            p_data.can_jump_timer += delta_time;
            if p_data.can_jump_timer >= p_data.jump_delay {
                p_data.state.clear(P::CanJump);
            }
        }

        if p_data.state.has(P::Jumping) {
            vel.y = -walker_d.jump_force;
            p_data.state.clear(P::Jumping);
            p_data.state.clear(P::CanJump);
        }
    }
}

pub fn player_fixed_update(ecs: &mut ECS, _time_step: f32) {
    use super::player_movement::*;
    for (_e,  p_data, walker_d, vel) in 
        ecs.query_comp::<(&mut PlayerData, &mut WalkerData, &mut Velocity)>() 
    {
        player_left_right_motion(p_data, walker_d, vel);

    }
}

pub fn player_input(ecs: &mut ECS, k_state: &mut KeyboardState) {
    for (_e, p_data) in ecs.query_comp::<&mut PlayerData>() {
        player_input_sys(p_data, k_state);
    }
}

fn player_input_sys(p_data: &mut PlayerData, k_state: &mut KeyboardState) {
    // p_data.running = false;
    p_data.run_dir = 0;

    if k_state.is_scancode_pressed(Scancode::Space) && p_data.state.has(P::CanJump) {
        p_data.state.set(P::Jumping);
    }

    if k_state.is_scancode_pressed(Scancode::A) {
        // p_data.running = true;
        p_data.run_dir = -1;
    }

    if k_state.is_scancode_pressed(Scancode::D) {
        // p_data.running = true;
        p_data.run_dir = 1;
    }
}
