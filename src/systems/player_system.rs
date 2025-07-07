use sdl2::keyboard::*; 
use crate::core::renderer::*;
use crate::components::animation_player::*;
use crate::components::sprite::*;
use crate::components::entity_data::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::components::area::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::asset_manager::*;
use crate::systems::player_animation_system::*;

#[derive(Default, Clone)]
pub struct PlayerTag {}

#[derive(Clone)]
pub struct PlayerInput {
    running: bool,
    pub run_dir: i32,
    jumping: bool,
    can_jump: bool,
    can_jump_timer: f32,
    jump_delay: f32,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            running: false,
            run_dir: 0,
            jumping: false,
            can_jump: false,
            can_jump_timer: 0.0,
            jump_delay: 0.05
        }
    }
}

// #[derive(Clone)]
// pub struct WalkerData {
//     run_speed: f32,
//     accel: f32,
//     jump_force: f32,
//     pub grounded: bool,
// }
// 
// impl Default for WalkerData {
//     fn default() -> Self {
//         Self {
//             run_speed: 200.0,
//             accel: 50.0,
//             jump_force: 300.0,
//             grounded: false,
//         }
//     }
// }

pub fn player_init(ecs: &mut ECS, renderer: &mut Renderer) {

    ecs.register_component::<PlayerTag>();
    ecs.register_component::<PlayerInput>();
    // ecs.register_component::<WalkerData>();

    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
    sprite.set_sprite_sheet(6, 6);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Position::new(12.0, 12.0);

    ecs.spawn::<(Sprite, Position, Velocity, Area, PlayerTag, PlayerInput, WalkerData, AnimationPlayer)>((
        sprite,
        Position { x: 10.0, y: -1000.0 },
        Velocity { x: 0.0, y: 0.0 },
        area,
        PlayerTag {},
        PlayerInput::default(),
        WalkerData::default(),
        AnimationPlayer::new(PAnims::COUNT),
    ));
}


pub fn player_update(ecs: &mut ECS, delta_time: f32) {
    for (_e, _p_tag, p_input, p_data, vel) in 
        ecs.query_comp::<(&PlayerTag, &mut PlayerInput, &WalkerData, &mut Velocity)>() {

        if p_data.grounded {
            p_input.can_jump = true;
            // reset jump timer set to false delay to 0
            p_input.can_jump_timer = 0.0;
        }
        // if not on ground, wait for jump_delay seconds before can_jump is disabled
        else {
            p_input.can_jump_timer += delta_time;
            if p_input.can_jump_timer >= p_input.jump_delay {
                p_input.can_jump = false;
            }
        }

        if p_input.jumping {
            vel.y = -p_data.jump_force;
            p_input.jumping = false;
            p_input.can_jump = false
        }
    }
}

pub fn player_fixed_update(ecs: &mut ECS, _time_step: f32) {
    for (_e, _p_tag, p_input, p_data, vel) in 
        ecs.query_comp::<(&PlayerTag, &mut PlayerInput, &WalkerData, &mut Velocity)>() {

        let mut run_dir: f32 = 0.0;
        // let mut vert_dir: f32 = 0.0;
        if p_input.run_dir == 1 {
            run_dir += 1.0;
        }
        if p_input.run_dir == -1 {
            run_dir -= 1.0;
        }
        // let run_speed = p_data.run_speed;
        if run_dir != 0.0 {
            vel.x += run_dir * p_data.accel;
            if vel.x.abs() >= p_data.run_speed {
                vel.x = p_data.run_speed.copysign(run_dir);
            }
        }
        else {
            if vel.x.abs() > 0.001 {
                vel.x -= p_data.accel.copysign(vel.x);
            }
            else {
                vel.x = 0.0;
            }
        }
    }
}

pub fn player_input(ecs: &mut ECS, k_state: &mut KeyboardState) {
    for (_e, p_input) in ecs.query_comp::<&mut PlayerInput>() {
        player_input_sys(p_input, k_state);
    }
}

fn player_input_sys(pi: &mut PlayerInput, k_state: &mut KeyboardState) {
    pi.running = false;
    pi.run_dir = 0;

    if k_state.is_scancode_pressed(Scancode::Space) && pi.can_jump {
        pi.jumping = true;
        // pi.can_jump = false;
    }

    if k_state.is_scancode_pressed(Scancode::A) {
        pi.running = true;
        pi.run_dir = -1;
    }

    if k_state.is_scancode_pressed(Scancode::D) {
        pi.running = true;
        pi.run_dir = 1;
    }
}
