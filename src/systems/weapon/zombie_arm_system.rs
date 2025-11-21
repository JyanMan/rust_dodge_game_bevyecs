use bevy_ecs::prelude::*;
use std::f64::consts::PI;

use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;

#[derive(Bundle)]
struct ZombieArmBundle {
    trans: Transform,
    sprite: Sprite,
    weapon_d: WeaponData,
    tag: ZombieArmTag,
    anim_player: AnimationPlayer,
    held: HeldBy,
    obb: OBB,
    cell_pos: CellPos,
    over_obbs: EntityOverlappingOBBs,
    target_tags: TargetEntityTags,
    tag_container: EntityTagContainer,
    funcs: WeaponFns,
}

pub fn zombie_arm_spawn(world: &mut World, renderer: &mut Renderer, entity_owner: Entity) -> Entity {

    let owner_ref = world.entity(entity_owner);
    let owner_tag = owner_ref.get::<EntityTagContainer>().unwrap();
    let entity_tag_container = EntityTagContainer(match owner_tag.0 {
        EntityTag::Zombie => EntityTag::EnemyWeapon,
        EntityTag::Player => EntityTag::PlayerWeapon,
        _ => EntityTag::Weapon,
    });
    
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::ZombieArm);
    sprite.set_sprite_sheet(2, 2);
    sprite.visible = true;

    let attack_dur = 0.2;

    let zombie_arm_e = world.spawn(ZombieArmBundle {
        trans: Transform::zero(),
        sprite,
        weapon_d: WeaponData::new(2, 800.0, attack_dur, 0.4, 0.1, WeaponState::Owned, WeaponType::ZombieArm), 
        tag: ZombieArmTag::default(),
        anim_player: AnimationPlayer::new(WeaponAnim::COUNT),
        held: HeldBy(entity_owner),
        obb: OBB::new(20.0, 20.0, Vector2::zero(), true),
        cell_pos: CellPos(Vec::new()),
        over_obbs: EntityOverlappingOBBs(Vec::new()),
        // target_entity_tags,
        target_tags: TargetEntityTags(vec![]),
        tag_container: entity_tag_container,
        funcs: WeaponFns {
            start_attack: zombie_arm_start_attack,
            while_attacking: zombie_arm_while_attacking,
            after_effect: zombie_arm_after_effect,
            end_attack: zombie_arm_end_attack,
        }
        // EntityTagContainer(EntityTag::Weapon),
    }).id();

    // init animation
    // do this separately as the pointer to sprite is moved on spawn
    let mut steelsword_ref = world.entity_mut(zombie_arm_e);
    let mut anim_player = steelsword_ref.get_mut::<AnimationPlayer>().unwrap();

    let attack_anim = Animation::new(attack_dur / 8.0, &[
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 0, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(0.0, -8.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 0, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(4.0, -4.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 1, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(9.0, -2.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 1, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(11.0, 0.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 2, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(11.0, 0.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 2, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(9.0, 2.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 3, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(4.0, 4.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 3, target: zombie_arm_e},
            AnimData::OBBOffset { offset: Vector2::new(0.0, 8.0), target: zombie_arm_e},
            AnimData::OBBUpdate { target: zombie_arm_e },
        ]),
    ]);

    anim_player.add_anim(WeaponAnim::Attack.usize(), attack_anim);

    zombie_arm_e
}

pub fn zombie_arm_animation(sprite: &mut Sprite, trans: &mut Transform, attack_dir: Vector2) {
    // flip y if left side 
    // this allows animation to be consistent not flipped on another direction
    if attack_dir.x < 0.0 {
        sprite.flip_y = true;
    }
    else if attack_dir.x > 0.0 {
        sprite.flip_y = false;
    }

    // convert normalized vec to ang in deg
    let angle_to_mouse = attack_dir.y.atan2(attack_dir.x);
    let angle_deg = angle_to_mouse as f64 * (180.0 / PI);
    // adjust sprite angle
    sprite.angle = angle_deg;

    let attack_range: f32 = 3.0;
    trans.local = attack_dir * attack_range;
}

pub fn zombie_arm_per_frame_update(weapon_d: &WeaponData, obb: &mut OBB) {
    let attack_dir = weapon_d.attack_dir;

    let angle_to_mouse = attack_dir.y.atan2(attack_dir.x);

    obb.rotation = angle_to_mouse;
    obb.rotate_around(Vector2::zero());
    obb.compute_vertices();
}

pub fn zombie_arm_while_attacking(
    _: &mut WeaponData,
    _: &mut GravityAffected,
    user_vel: &mut Velocity,
    _: &mut Combat,
    _: &mut Sprite,
    _: &mut Transform,
    anim_player: &mut AnimationPlayer
    // attack_dir: Vector2,
) {
    anim_player.play(WeaponAnim::Attack.usize());
    user_vel.vec = user_vel.vec * 0.5;
}
pub fn zombie_arm_after_effect(
    _: &mut WeaponData,
    _: &mut GravityAffected,
    user_vel: &mut Velocity,
    _: &mut Combat,
    _: &mut Sprite,
    _: &mut Transform,
    anim_player: &mut AnimationPlayer
) {
    user_vel.vec = user_vel.vec * 0.2;
    anim_player.stop();
}

// fn(&mut WeaponData, &mut GravityAffected, &mut Velocity, &mut Combat, &mut Sprite, &mut Transform)
pub fn zombie_arm_start_attack(
    weapon_d: &mut WeaponData,
    grav_affected: &mut GravityAffected,
    user_vel: &mut Velocity,
    combat: &mut Combat,
    sprite: &mut Sprite,
    trans: &mut Transform,
    anim_player: &mut AnimationPlayer
    // attack_dir: Vector2,
) {
    let attack_dir = combat.attack_dir;
    zombie_arm_animation(sprite, trans, attack_dir);
    anim_player.play(WeaponAnim::Attack.usize());

    weapon_d.attack_dir = attack_dir;
    user_vel.vec = attack_dir * 200.0;
    grav_affected.0 = false;
    combat.attacking = true;
}

pub fn zombie_arm_end_attack(
    _: &mut WeaponData,
    grav_affected: &mut GravityAffected,
    user_vel: &mut Velocity,
    combat: &mut Combat,
    _: &mut Sprite,
    _: &mut Transform,
    _: &mut AnimationPlayer
) {
    // anim_player.stop();
    user_vel.vec = Vector2::zero();
    grav_affected.0 = true;
    combat.attacking = false;
}

#[allow(dead_code)]
pub fn zombie_arm_test_overlap(
    mut query: Query<(&ZombieArmTag, &EntityOverlappingOBBs)>, 
) {
    for (_e, e_over_obbs) in &mut query {
        if !e_over_obbs.0.is_empty() {
            println!("sword is overlapping");
        }
    }
}
