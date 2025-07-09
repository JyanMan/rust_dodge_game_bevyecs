use rand::*;
use crate::core::renderer::*;
use crate::components::animation_player::*;
use crate::components::animation::*;
use crate::components::entity::*;
use crate::components::sprite::*;
// use crate::components::state_machine::*;
use crate::components::walker_data::*;
use crate::components::walker_state::*;
use crate::components::walker_animation::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::components::area::*;
use crate::ecs::ecs::*;
use crate::managers::asset_manager::*;

pub fn zombie_register_components(ecs: &mut ECS) {
    ecs.register_component::<ZombieTag>();
}

pub fn zombie_init(ecs: &mut ECS, renderer: &mut Renderer) {
    let mut rng = rand::thread_rng(); 
    for _ in 0..2 {
        zombie_spawn(ecs, renderer, rng.gen_range(30..80) as f32);
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
            state: WalkerState::default(),
        },
        AnimationPlayer::new(WalkerAnim::COUNT),
        // StateMachine::default(),
    ));
}

pub fn zombie_fixed_update(ecs: &mut ECS, _time_step: f32) {
    let mut p_pos = Position::zero();
    for (_e, _p_tag, pos) in ecs.query_comp::<(&PlayerData, &Position)>() {
        p_pos = *pos;
    }
    for (_e, pos, vel, _z_tag, walker_d) 
        in ecs.query_comp::<(&Position, &mut Velocity, &ZombieTag, &mut WalkerData)>() 
    {
        // jump ai
        if vel.vec.x.abs() <= 0.001 && walker_d.state == WalkerState::Chasing {
            vel.vec.y -= walker_d.jump_force;
        }

        let x_pos = p_pos.vec.x - pos.vec.x;
        let y_pos = p_pos.vec.y - pos.vec.y;

        // calc dist
        let mut dist = (x_pos*x_pos + y_pos*y_pos).sqrt();
        // disallow dividing by zero
        if dist == 0.0 {
            dist = 0.0001;
        }

        // get the direction on x axis
        let x_dir = {1.0 as f32}.copysign(x_pos);

        // move toward dir if on distance or far away
        if dist <= 200.0 && dist >= 20.0 {
            walker_d.state = WalkerState::Chasing;
            vel.vec.x += x_dir * walker_d.accel;
        }
        else {
            walker_d.state = WalkerState::Idle;
            vel.vec.x -= x_dir.copysign(vel.vec.x) * walker_d.accel;
            if vel.vec.x.abs() <= walker_d.accel {
                vel.vec.x = 0.0;
            }
        }
        if vel.vec.x.abs() >= walker_d.run_speed {
            vel.vec.x = walker_d.run_speed.copysign(vel.vec.x);
        }
    }    
}

// #[derive(Clone)]
// #[repr(usize)]
// pub enum ZomAnim {
//     // these are indeces for animation lookup
//     Idle,
//     Run,
//     Rise,
//     Fall
// }
// 
// impl ZomAnim {
//     pub const COUNT: usize = 4;
//     pub fn usize(&self) -> usize {
//         self.clone() as usize
//     }
// }

pub fn zombie_animation_init(ecs: &mut ECS, _renderer: &mut Renderer) {
    for (_e, sprite, anim_player, _z_tag) in 
        ecs.query_comp::<(&mut Sprite, &mut AnimationPlayer, &ZombieTag)>() {

        let s_frame_ptr = &mut sprite.frame as *mut _;

        let mut idle_anim = Animation::new(3, 0.2);
        idle_anim.set_frame(0, AnimData::Integer { value: 0, target: s_frame_ptr});
        idle_anim.set_frame(1, AnimData::Integer { value: 1, target: s_frame_ptr});
        idle_anim.set_frame(2, AnimData::Integer { value: 2, target: s_frame_ptr});

        let mut run_anim = Animation::new(5, 0.1);
        run_anim.set_frame(0, AnimData::Integer { value: 3, target: s_frame_ptr});
        run_anim.set_frame(1, AnimData::Integer { value: 4, target: s_frame_ptr});
        run_anim.set_frame(2, AnimData::Integer { value: 5, target: s_frame_ptr});
        run_anim.set_frame(3, AnimData::Integer { value: 6, target: s_frame_ptr});
        run_anim.set_frame(4, AnimData::Integer { value: 7, target: s_frame_ptr});

        let mut rise_anim = Animation::new(1, 0.2);
        rise_anim.set_frame(0, AnimData::Integer { value: 8, target: s_frame_ptr});

        let mut fall_anim = Animation::new(1, 0.2);
        fall_anim.set_frame(0, AnimData::Integer { value: 9, target: s_frame_ptr});

        anim_player.add_anim(WalkerAnim::Idle.usize(), idle_anim);
        anim_player.add_anim(WalkerAnim::Run.usize(), run_anim);
        anim_player.add_anim(WalkerAnim::Rise.usize(), rise_anim);
        anim_player.add_anim(WalkerAnim::Fall.usize(), fall_anim);
        // anim_player.play(PAnims::Run.usize());
    }
}

