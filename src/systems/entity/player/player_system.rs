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

pub fn player_init(ecs: &mut ECS, renderer: &mut Renderer) {

    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Position::new(12.0, 12.0);

    ecs.spawn::<(Sprite, Position, Velocity, Area, PlayerData, PlayerInput, WalkerData, AnimationPlayer)>((
        sprite,
        Position::new(10.0, -1000.0),
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
        // StateMachine::default(),
    ));
}

pub fn player_update(ecs: &mut ECS, delta_time: f32) {
    use super::player_timer::*;
    for (_e, p_data, walker_d) in 
        ecs.query_comp::<(&mut PlayerData, &WalkerData)>() 
    {
        player_can_jump_delay_timer(p_data, walker_d, delta_time);
        player_can_dodge_timer(p_data, delta_time);
    }
}

pub fn player_fixed_update(ecs: &mut ECS, _time_step: f32) {
    use super::player_movement::*;
    for (_e,  p_data, walker_d, pos, vel, input) in 
        ecs.query_comp::<(&mut PlayerData, &mut WalkerData, &Position, &mut Velocity, &PlayerInput)>() 
    {
        if input.dodge && p_data.can_dodge {
            player_dodge(p_data, walker_d, vel, pos);
        }

        player_left_right_motion(p_data, walker_d, vel, input);

        if input.jump && p_data.can_jump {
            player_jump(p_data, walker_d, vel);
        }
    }
}

pub fn player_input(ecs: &mut ECS, k_state: &mut KeyboardState) {
    use super::player_input::*;
    for (_e, input) in ecs.query_comp::<&mut PlayerInput>() {
        player_input_sys(input, k_state);
    }
}

