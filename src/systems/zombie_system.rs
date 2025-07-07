use sdl2::keyboard::*; 
use rand::*;
use crate::core::renderer::*;
use crate::components::animation_player::*;
use crate::components::sprite::*;
use crate::components::entity_data::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::components::area::*;
use crate::systems::player_system::*;
use crate::ecs::ecs::*;
use crate::managers::asset_manager::*;

#[derive(Clone, Default)]
struct ZombieTag {}

pub fn zombie_register_components(ecs: &mut ECS) {
    ecs.register_component::<ZombieTag>();
}

pub fn zombie_init(ecs: &mut ECS, renderer: &mut Renderer) {
    let mut rng = rand::thread_rng(); 
    for _ in 0..3000 {
        zombie_spawn(ecs, renderer, rng.gen_range(40..100) as f32);
    }
}

pub fn zombie_spawn(ecs: &mut ECS, renderer: &mut Renderer, speed: f32) {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Zombie);
    sprite.set_sprite_sheet(4, 4);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Position::new(12.0, 12.0);
    ecs.spawn::<(ZombieTag, Position, Velocity, Area, Sprite, WalkerData, AnimationPlayer)>((
        ZombieTag {},
        Position::new(10.0, -1000.0),
        Velocity::zero(),
        area,
        sprite,
        WalkerData {
            grounded: false,
            jump_force: 200.0,
            run_speed: speed,
            accel: speed / 10.0,
        },
        AnimationPlayer::new(4),
    ));
}

pub fn zombie_fixed_update(ecs: &mut ECS, _time_step: f32) {
    let mut p_pos = Position::zero();
    for (_e, _p_tag, pos) in ecs.query_comp::<(&PlayerTag, &Position)>() {
        p_pos = *pos;
    }
    for (_e, pos, vel, _z_tag, z_data) 
        in ecs.query_comp::<(&Position, &mut Velocity, &ZombieTag, &WalkerData)>() {

        if vel.x.abs() <= 0.001 && z_data.grounded {
            vel.y -= z_data.jump_force;
            //zombie_jump(vel, z_data);
        }

        let x_pos = p_pos.x - pos.x;
        let y_pos = p_pos.y - pos.y;
        let mut dist = (x_pos*x_pos + y_pos*y_pos).sqrt();
        if dist == 0.0 {
            dist = 0.0001;
        }
        let x_dir = x_pos / dist;
        if dist <= 200.0 && dist >= 10.0 {
            vel.x += x_dir * z_data.accel;
        }
        else {
            vel.x -= x_dir.copysign(vel.x) * z_data.accel;
            if vel.x.abs() <= z_data.accel {
                vel.x = 0.0;
            }
        }
        if vel.x.abs() >= z_data.run_speed {
            vel.x = z_data.run_speed.copysign(vel.x);
        }
    }    
}
