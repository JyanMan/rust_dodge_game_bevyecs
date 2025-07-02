use sdl2::event::Event;
use sdl2::keyboard::Keycode; 
use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::sprite::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::components::area::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::asset_manager::*;

#[derive(Default, Clone)]
pub struct PlayerTag {}

#[derive(Clone)]
pub struct PlayerInput {
    running: bool,
    run_dir: i32,
    jumping: bool,
    can_jump: bool,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            running: false,
            run_dir: 0,
            jumping: false,
            can_jump: true,
        }
    }
}

#[derive(Clone)]
pub struct PlayerData {
    run_speed: f32,
    accel: f32,
    jump_force: f32,
    //TEMP
    pub grounded: bool,
}

impl Default for PlayerData {
    fn default() -> Self {
        Self {
            run_speed: 500.0,
            accel: 20.0,
            jump_force: 300.0,
            grounded: false,
        }
    }
}

pub fn player_startup_system() -> StartFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {

        let player = ecs.create_entity();
        let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Player);
        sprite.set_sprite_sheet(6, 6);

        let mut area = Area::new(
            10.0, -1000.0, 10.0, 20.0
        );
        area.offset = Position::new(12.0, 12.0);

        ecs.register_component::<PlayerTag>();
        ecs.register_component::<PlayerInput>();
        ecs.register_component::<PlayerData>();

        ecs.add_component::<Sprite>(player, sprite);
        ecs.add_component::<Position>(player, Position { x: 10.0, y: -1000.0 });
        ecs.add_component::<Velocity>(player, Velocity { x: 0.0, y: 0.0 });
        ecs.add_component::<Area>(player, area);

        ecs.add_component::<PlayerTag>(player, PlayerTag {});
        ecs.add_component::<PlayerInput>(player, PlayerInput::default());
        ecs.add_component::<PlayerData>(player, PlayerData::default());
    })
}

pub fn player_update_system() -> UpdateFn {
    Box::new(|ecs: &mut ECS, _delta_time: f32| {
        let _entities = ecs.query_entities(&[
            TypeId::of::<PlayerTag>(),
            TypeId::of::<PlayerInput>(),
            TypeId::of::<PlayerData>(),
        ]);

        // for e in entities {
        //     if let (Some(_p_tag), Some(p_input), Some(p_data)) = (
        //         ecs.get_component::<PlayerTag>(e),
        //         ecs.get_component::<PlayerInput>(e),
        //         ecs.get_component::<PlayerData>(e),
        //     ) {
        //         let mut run_dir: f32 = 0.0;
        //         // let mut vert_dir: f32 = 0.0;
        //         if p_input.run_dir == 1 {
        //             run_dir += 1.0;
        //         }
        //         if p_input.run_dir == -1 {
        //             run_dir -= 1.0;
        //         }
        //         let run_speed = p_data.run_speed;
        //         if let Some(vel) = ecs.get_component_mut::<Velocity>(e) {
        //             vel.x = run_dir * run_speed;
        //             // vel.y = vert_dir * run_speed;
        //         }
        //     }
        // }
    })
}

pub fn player_fixed_update_system() -> FixedUpdateFn {
    Box::new(|ecs: &mut ECS, _time_step: f32| {
        let entities = ecs.query_entities(&[
            TypeId::of::<PlayerTag>(),
            TypeId::of::<PlayerInput>(),
            TypeId::of::<PlayerData>(),
        ]);

        for e in entities {
            if let (Some(_p_tag), Some(p_input), Some(p_data)) = (
                ecs.get_component::<PlayerTag>(e),
                ecs.get_component::<PlayerInput>(e),
                ecs.get_component::<PlayerData>(e),
            ) {
                let mut run_dir: f32 = 0.0;
                // let mut vert_dir: f32 = 0.0;
                if p_input.run_dir == 1 {
                    run_dir += 1.0;
                }
                if p_input.run_dir == -1 {
                    run_dir -= 1.0;
                }
                let run_speed = p_data.run_speed;
                if let Some(vel) = ecs.get_component_mut::<Velocity>(e) {
                    vel.x = run_dir * run_speed;
                    // vel.y = vert_dir * run_speed;
                }
            }
        }
    })
}

pub fn player_input_system() -> InputFn {
    Box::new(|ecs: &mut ECS, event: &Event| {
        let entities = ecs.query_entities(&[
            TypeId::of::<PlayerInput>(),
        ]);

        for e in entities {
            if let Some(player_input) = ecs.get_component_mut::<PlayerInput>(e)
            {
                player_input_sys(player_input, event);
            }
        }
    })
}

fn player_input_sys(pi: &mut PlayerInput, event: &Event) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
            pi.running = true; 
            pi.run_dir = -1;
        },
        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
            pi.running = true; 
            pi.run_dir = 1;
        },
        Event::KeyUp { keycode: Some(Keycode::A), .. } => {
            pi.running = false; 
            pi.run_dir = 0;
        },
        Event::KeyUp { keycode: Some(Keycode::D), .. } => {
            pi.running = false; 
            pi.run_dir = 0;
        },
        _ => {}
    }
}
