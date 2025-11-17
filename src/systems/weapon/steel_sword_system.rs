use bevy_ecs::prelude::*;
use std::f64::consts::PI;

use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;

pub fn steel_sword_spawn(world: &mut World, renderer: &mut Renderer, entity_owner: Entity) -> Entity {

    let owner_ref = world.entity(entity_owner);
    let owner_tag = owner_ref.get::<EntityTagContainer>().unwrap();
    let entity_tag_container = EntityTagContainer(match owner_tag.0 {
        EntityTag::Zombie => EntityTag::EnemyWeapon,
        EntityTag::Player => EntityTag::PlayerWeapon,
        _ => EntityTag::Weapon,
    });

    
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::SteelSword);
    sprite.set_sprite_sheet(4, 2);
    sprite.visible = true;

    let attack_dur = 0.2;

    let steel_sword_e = world.spawn((
        Transform::zero(),
        sprite,
        WeaponData::new(1, 800.0, attack_dur, WeaponState::Owned, WeaponType::SteelSword), 
        SteelSwordTag::default(),
        AnimationPlayer::new(WeaponAnim::COUNT),
        HeldBy(entity_owner),
        OBB::new(30.0, 30.0, Vector2::zero(), true),
        CellPos(Vec::new()),
        EntityOverlappingOBBs(Vec::new()),
        // target_entity_tags,
        TargetEntityTags(vec![]),
        entity_tag_container,
        // EntityTagContainer(EntityTag::Weapon),
    )).id();

    // init animation
    // do this separately as the pointer to sprite is moved on spawn
    let mut steelsword_ref = world.entity_mut(steel_sword_e);
    let mut anim_player = steelsword_ref.get_mut::<AnimationPlayer>().unwrap();

    let attack_anim = Animation::new(attack_dur / 8.0, &[
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 0, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(0.0, -20.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 0, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(8.0, -13.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 1, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(14.0, -6.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 1, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(18.0, 0.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 2, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(18.0, 0.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 2, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(14.0, 6.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 3, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(8.0, 13.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
        
        AnimFrame::new(&[
            AnimData::SpriteFrame { value: 3, target: steel_sword_e},
            AnimData::OBBOffset { offset: Vector2::new(0.0, 20.0), target: steel_sword_e},
            AnimData::OBBUpdate { target: steel_sword_e },
        ]),
    ]);

    anim_player.add_anim(WeaponAnim::Attack.usize(), attack_anim);

    steel_sword_e
}

pub fn steel_sword_animation(sprite: &mut Sprite, trans: &mut Transform, attack_dir: Vector2) {
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

pub fn steel_sword_per_frame_update(weapon_d: &WeaponData, obb: &mut OBB) {
    let attack_dir = weapon_d.attack_dir;

    let angle_to_mouse = attack_dir.y.atan2(attack_dir.x);

    obb.rotation = angle_to_mouse;
    obb.rotate_around(Vector2::zero());
    obb.compute_vertices();
}

pub fn steel_sword_start_attack_effect(user_vel: &mut Velocity, attack_dir: Vector2, grav_affected: &mut GravityAffected) {
    user_vel.vec = attack_dir * 1000.0;
    grav_affected.0 = false;
}

pub fn steel_sword_end_attack_effect(user_vel: &mut Velocity, grav_affected: &mut GravityAffected) {
    user_vel.vec = Vector2::zero();
    grav_affected.0 = true;
}

#[allow(dead_code)]
pub fn steel_sword_test_overlap(
    mut query: Query<(&SteelSwordTag, &EntityOverlappingOBBs)>, 
) {
    for (_e, e_over_obbs) in &mut query {
        if !e_over_obbs.0.is_empty() {
            println!("sword is overlapping");
        }
    }
}
