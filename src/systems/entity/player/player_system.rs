use sdl2::EventPump;
use sdl2::keyboard::*;
use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::entity::{ WalkerData, WalkerState, WalkerAnim, };
use crate::components::*;
use crate::ecs::ecs::*;
use crate::resources::asset_manager::*;
use crate::resources::event_pump_res::UserInputRes;
use crate::resources::delta_time_res::DeltaTimeRes;
use crate::components::entity::*;
use crate::systems::weapon::*;

use PlayerState as P;

pub fn player_timer_system(mut query: Query<(&mut PlayerData, &mut WalkerData)>, dt_res: Res<DeltaTimeRes>) {
    use super::player_timer::*;
    for (mut p_data, mut walker_d) in &mut query {
        let delta_time = dt_res.delta_time;
        player_can_jump_delay_timer(&mut p_data, &mut walker_d, delta_time);
        player_dodge_timer(&mut p_data, delta_time);
        player_lerp_timer(&mut p_data, delta_time);
    } 
}

pub fn player_init(ecs: &mut ECS, renderer: &mut Renderer) {

    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);

    let player = ecs.spawn::<(
        Sprite, Transform, Velocity, Area, 
        PlayerData, PlayerInput, WalkerData, AnimationPlayer,
        Combat,
        )>((
        sprite,
        Transform::new(10.0, -1000.0),
        Velocity::new(0.0, 0.0),
        area,
        PlayerData::default(),
        PlayerInput::default(),
        WalkerData {
            run_speed: 200.0,
            accel: 50.0,
            jump_force: 300.0,
            grounded: false,
            state: WalkerState::default()
        },
        AnimationPlayer::new(WalkerAnim::COUNT),
        Combat::default(),
    ));

    // new_steel_sword(ecs, renderer, player);
}

pub fn player_update(ecs: &mut ECS, delta_time: f32) {
    use super::player_timer::*;
    for (_e, p_data, walker_d) in 
        ecs.query_comp::<(&mut PlayerData, &WalkerData)>() 
    {
        player_can_jump_delay_timer(p_data, walker_d, delta_time);
        player_dodge_timer(p_data, delta_time);
        player_lerp_timer(p_data, delta_time);
    }
}

pub fn player_movement_system(mut query: Query<(&mut PlayerData, &mut WalkerData, &mut Velocity, &PlayerInput)>, user_input: Res<UserInputRes>) {
    use super::player_movement::*;

    let mouse_pos = user_input.mouse_pos;

    for (mut p_data, mut walker_d, mut vel, input) in &mut query {
        if input.dodge && p_data.can_dodge {
            player_dodge(&mut p_data);
        }

        // only allow dodge again if dodge button is let go
        // otherwise, player can fly like superman
        if p_data.state != P::Dodging && !input.dodge {
            p_data.can_dodge = true; 
        }
        if p_data.state == P::Dodging {
            let dodge_dir = get_dodge_dir(mouse_pos, &mut p_data);
            player_dodging(dodge_dir, &mut p_data, &mut vel);
            return;
        } 
        else if p_data.state == P::Lerping {
            player_lerping(&mut vel);
            return;
        }

        player_left_right_motion(&mut p_data, &mut walker_d, &mut vel, input);

        if input.jump && p_data.can_jump {
            player_jump(&mut p_data, &mut walker_d, &mut vel);
        }
    }
}

