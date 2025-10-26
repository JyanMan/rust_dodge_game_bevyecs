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

pub fn player_spawn(world: &mut World, renderer: &mut Renderer) {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);

    let player_e = world.spawn((
         sprite,
         Transform::new(10.0, -1000.0),
         Velocity::new(0.0, 0.0),
         area,
         PlayerData::default(),
         PlayerTag {},
         PlayerInput::default(),
         WalkerData {
             run_speed: 200.0,
             accel: 50.0,
             jump_force: 300.0,
             grounded: false,
             state: WalkerState::default()
         },
         // AnimationPlayer::new(WalkerAnim::COUNT),
         Combat::default(),
    )).id();
}

pub fn player_system_input(mut query: Query<(&mut PlayerInput, &mut Combat)>, user_input_res: Res<UserInputRes>) {
    for (mut input, mut combat) in &mut query {
        if user_input_res.k_state.contains(&Keycode::Space) { input.jump = true; } else { input.jump = false; }

        if user_input_res.k_state.contains(&Keycode::A) { input.left = true; println!("pressed a") } else { input.left = false; }

        if user_input_res.k_state.contains(&Keycode::D) { input.right = true; } else { input.right = false; }

        if user_input_res.k_state.contains(&Keycode::Q) { input.dodge = true; } else { input.dodge = false; }

        if user_input_res.k_state.contains(&Keycode::E) { combat.attacking = true } else { combat.attacking = false }
    }
}

pub fn player_input_system(mut query: Query<(&mut PlayerInput, &mut Combat)>, user_input: ResMut<UserInputRes>) {
    use super::player_input::*;
    for (mut input, mut combat) in &mut query {
        player_input_update(&mut input, &user_input.k_state, &mut combat);
    }
}

pub fn player_movement_system(mut query: Query<(&mut PlayerData, &mut WalkerData)>, dt_res: Res<DeltaTimeRes>) {
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

    new_steel_sword(ecs, renderer, player);
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

pub fn player_fixed_update(ecs: &mut ECS, _time_step: f32) {
    use super::player_movement::*;

    for (_e,  p_data, walker_d, vel, input) in 
        ecs.query_comp::<(&mut PlayerData, &mut WalkerData, &mut Velocity, &PlayerInput)>() 
    {
        if input.dodge && p_data.can_dodge {
            player_dodge(p_data);
        }

        // only allow dodge again if dodge button is let go
        // otherwise, player can fly like superman
        if p_data.state != P::Dodging && !input.dodge {
            p_data.can_dodge = true; 
        }
        if p_data.state == P::Dodging {
            let dodge_dir = get_dodge_dir(ecs, p_data);
            player_dodging(dodge_dir, p_data, vel);
            return;
        } 
        else if p_data.state == P::Lerping {
            player_lerping(vel);
            return;
        }

        player_left_right_motion(p_data, walker_d, vel, input);

        if input.jump && p_data.can_jump {
            player_jump(p_data, walker_d, vel);
        }
    }
}

pub fn player_input(ecs: &mut ECS, k_state: &mut EventPump) {
    use super::player_input::*;
    for (_e, input, combat) in ecs.query_comp::<(&mut PlayerInput, &mut Combat)>() {
        player_input_sys(input, k_state, combat);
    }
}

