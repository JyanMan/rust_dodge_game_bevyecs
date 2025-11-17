use rand::*;
use bevy_ecs::prelude::*;
use crate::core::renderer::*;
use crate::components::*;
// use crate::ecs::ecs::*;
use crate::resources::*;
use crate::systems::*;

pub fn zombie_init(world: &mut World, renderer: &mut Renderer) {
    let mut rng = rand::thread_rng(); 
    for _ in 0..300 {
        let z = zombie_spawn(world, renderer, rng.gen_range(30..80) as f32);
        steel_sword_spawn(world, renderer, z);
    }
}

#[derive(Bundle)]
struct ZombieBundle {
    zom_tag: ZombieTag,
    enemy_tag: EnemyTag,
    trans: Transform,
    grav_affected: GravityAffected,
    vel: Velocity,
    area: Area,
    sprite: Sprite,
    walker_d: WalkerData,
    anim_player: AnimationPlayer,
    health: Health,
    obb: OBB,
    enemy_d: EnemyData,
    cell_pos: CellPos,
    combat: Combat,
    e_over_obbs: EntityOverlappingOBBs,
    target_e_tags: TargetEntityTags,
    tag_container: EntityTagContainer,
    knock: KnockbackTrigger
}

pub fn zombie_spawn(world: &mut World, renderer: &mut Renderer, speed: f32) -> Entity {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::Zombie);
    sprite.set_sprite_sheet(4, 4);

    let mut area = Area::new(
        10.0, -1000.0, 10.0, 20.0
    );
    area.offset = Vector2::new(0.0, 6.0);
    let zombie_e = world.spawn(ZombieBundle {
        zom_tag: ZombieTag {},
        enemy_tag: EnemyTag {},
        trans: Transform::new(10.0, -1000.0),
        grav_affected: GravityAffected(true),
        vel: Velocity::zero(),
        area,
        sprite,
        walker_d: WalkerData {
            grounded: false,
            jump_force: 200.0,
            run_speed: speed,
            accel: speed / 10.0,
            state: WalkerState::default(),
        },
        anim_player: AnimationPlayer::new(WalkerAnim::COUNT),
        health: Health::new(10),
        obb: OBB::new(10.0, 20.0, Vector2::new(10.0, -1000.0), false),
        enemy_d: EnemyData { chase_range: 200.0, attack_range: 20.0},
        cell_pos: CellPos(Vec::new()),
        combat: Combat::new(1.0),
        e_over_obbs: EntityOverlappingOBBs(Vec::new()),
        target_e_tags: TargetEntityTags(vec![EntityTag::PlayerWeapon]),
        tag_container: EntityTagContainer(EntityTag::Zombie),
        knock: KnockbackTrigger::default()
        // StateMachine::default(),
    }).id();

    let mut zombie_ref = world.entity_mut(zombie_e);
    let mut anim_player = zombie_ref.get_mut::<AnimationPlayer>().unwrap();
    zombie_animation_init(&mut anim_player, zombie_e);

    zombie_e
}

pub fn zombie_movement_system(
    player_query: Query<(&PlayerTag, &Transform)>, 
    mut query: Query<(&Transform, &mut Velocity, &ZombieTag, &mut WalkerData,  &EnemyData, &KnockbackTrigger, &mut Combat)>,
) {
    let mut p_trans = Transform::zero();
    for (_p_tag, trans) in &player_query {
        p_trans = *trans;
    }
    for (trans, mut vel, _z_tag, mut walker_d, enemy_d, knock, mut combat) in &mut query {
        if knock.knocked { continue; }
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
        let dir_to_player = Vector2::new(x_trans, y_trans).normalize();
        let x_dir = (1.0 as f32).copysign(x_trans);

        combat.attacking = false;
        if dist <= enemy_d.chase_range && dist >= enemy_d.attack_range {
            walker_d.state = WalkerState::Running;
            vel.vec.x += x_dir * walker_d.accel;
        }
        else {
            // attack
            if walker_d.grounded {
                combat.attacking = true;
                combat.attack_dir = dir_to_player;
            }
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
    let idle_anim = Animation::new(0.2, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 0, target: zombie_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 1, target: zombie_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 2, target: zombie_e} ] ),
    ]);

    let run_anim = Animation::new(0.1, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 3, target: zombie_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 4, target: zombie_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 5, target: zombie_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 6, target: zombie_e} ] ),
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 7, target: zombie_e} ] ),
    ]);

    let rise_anim = Animation::new(0.2, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 8, target: zombie_e} ] ),
    ]);

    let fall_anim = Animation::new(0.2, &[
        AnimFrame::new(&[ AnimData::SpriteFrame { value: 9, target: zombie_e} ] ),
    ]);

    anim_player.add_anim(WalkerAnim::Idle.usize(), idle_anim);
    anim_player.add_anim(WalkerAnim::Run.usize(), run_anim);
    anim_player.add_anim(WalkerAnim::Rise.usize(), rise_anim);
    anim_player.add_anim(WalkerAnim::Fall.usize(), fall_anim);
}

