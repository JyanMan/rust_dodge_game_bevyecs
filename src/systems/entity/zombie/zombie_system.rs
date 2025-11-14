use rand::*;
use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::*;
// use crate::ecs::ecs::*;
use crate::resources::*;
use crate::math_helper::*;

pub fn zombie_init(world: &mut World, renderer: &mut Renderer) {
    let mut rng = rand::thread_rng(); 
    for _ in 0..150 {
        zombie_spawn(world, renderer, rng.gen_range(30..80) as f32);
    }
}

pub fn zombie_spawn(world: &mut World, renderer: &mut Renderer, speed: f32) {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Zombie);
    sprite.set_sprite_sheet(4, 4);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);
    let zombie_e = world.spawn((
        ZombieTag {},
        Transform::new(10.0, -1000.0),
        GravityAffected(true),
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
        OBB::new(10.0, 20.0, Vector2::new(10.0, -1000.0)),
        EnemyData { chase_range: 200.0, attack_range: 20.0},
        CellPos(Vec::new()),
        EntityOverlappingOBBs { entities: Vec::new(), target_tags: vec![EntityTag::Player] },
        EntityTagContainer(EntityTag::Zombie),
        // StateMachine::default(),
    )).id();

    let mut zombie_ref = world.entity_mut(zombie_e);
    let mut anim_player = zombie_ref.get_mut::<AnimationPlayer>().unwrap();
    zombie_animation_init(&mut anim_player, zombie_e);
}

pub fn zombie_movement_system(
    player_query: Query<(&PlayerTag, &Transform)>, 
    mut query: Query<(&Transform, &mut Velocity, &ZombieTag, &mut WalkerData,  &EnemyData)>,
) {
    let mut p_trans = Transform::zero();
    for (_p_tag, trans) in &player_query {
        p_trans = *trans;
    }
    for (trans, mut vel, _z_tag, mut walker_d, enemy_d) in &mut query {
        // jump ai
        if vel.vec.x.abs() <= 0.001 && walker_d.state == WalkerState::Running {
            vel.vec.y -= walker_d.jump_force;
        }

        let x_trans = p_trans.global.x - trans.global.x;
        let y_trans = p_trans.global.y - trans.global.y;

        // calc dist
        let mut dist = (x_trans*x_trans + y_trans*y_trans).sqrt();
        // disallow dividing by zero
        if dist == 0.0 {
            dist = 0.0001;
        }

        // get the direction on x axis
        let x_dir = {1.0 as f32}.copysign(x_trans);

        if dist <= enemy_d.chase_range && dist >= enemy_d.attack_range {
            walker_d.state = WalkerState::Running;
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

pub fn zombie_animation_init(anim_player: &mut AnimationPlayer, zombie_e: Entity) {
    let mut idle_anim = Animation::new(3, 0.2);
    idle_anim.set_frame(0, AnimData::SpriteFrame { value: 0, target: zombie_e});
    idle_anim.set_frame(1, AnimData::SpriteFrame { value: 1, target: zombie_e});
    idle_anim.set_frame(2, AnimData::SpriteFrame { value: 2, target: zombie_e});

    let mut run_anim = Animation::new(5, 0.1);
    run_anim.set_frame(0, AnimData::SpriteFrame { value: 3, target: zombie_e});
    run_anim.set_frame(1, AnimData::SpriteFrame { value: 4, target: zombie_e});
    run_anim.set_frame(2, AnimData::SpriteFrame { value: 5, target: zombie_e});
    run_anim.set_frame(3, AnimData::SpriteFrame { value: 6, target: zombie_e});
    run_anim.set_frame(4, AnimData::SpriteFrame { value: 7, target: zombie_e});

    let mut rise_anim = Animation::new(1, 0.2);
    rise_anim.set_frame(0, AnimData::SpriteFrame { value: 8, target: zombie_e});

    let mut fall_anim = Animation::new(1, 0.2);
    fall_anim.set_frame(0, AnimData::SpriteFrame { value: 9, target: zombie_e});

    anim_player.add_anim(WalkerAnim::Idle.usize(), idle_anim);
    anim_player.add_anim(WalkerAnim::Run.usize(), run_anim);
    anim_player.add_anim(WalkerAnim::Rise.usize(), rise_anim);
    anim_player.add_anim(WalkerAnim::Fall.usize(), fall_anim);
    // anim_player.play(PAnims::Run.usize());
}

