use bevy_ecs::prelude::*;
use std::f64::consts::PI;
use std::any::TypeId;

use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;
use crate::components::states::*;
use crate::bundles::*;

#[derive(Bundle)]
struct SteelSwordBundle {
    sprite: Sprite,
    // weapon_d: WeaponData,
    tag: SteelSwordTag,
    anim_player: AnimationPlayer,
    held: HeldBy,
    weapon: WeaponBundle,
    test_only: StatusInflictor<DamageOverTime> 
}

// TODO: HeldBy shouldn't be set on spawn, it should be set dynamically if someone's holding the weapon
pub fn spawn(world: &mut World, entity_owner: Entity) -> Entity {

    let renderer = world.get_non_send_resource::<Renderer>().unwrap();

    let owner_ref = world.entity(entity_owner);
    // let owner_tag = owner_ref.get::<EntityTagContainer>().unwrap();
    // let entity_tag_container = EntityTagContainer(match owner_tag.0 {
    //     EntityTag::Zombie => EntityTag::EnemyWeapon,
    //     EntityTag::Player => EntityTag::PlayerWeapon,
    //     _ => EntityTag::Weapon,
    // });

    
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::SteelSword);
    sprite.set_sprite_sheet(4, 2);
    sprite.visible = true;

    let attack_dur = 0.2;

    let steel_sword_e = world.spawn(SteelSwordBundle {
        sprite,
        tag: SteelSwordTag,
        anim_player: AnimationPlayer::new(WeaponAnim::COUNT),
        held: HeldBy(entity_owner),
        weapon: WeaponBundle {
            tag: WeaponTag,
            trans: Transform::zero(),
            hitbox: HitboxBundle {
                obb: OBB::new(30.0, 30.0, Vector2::zero(), true),
                cell_pos: CellPos(Vec::new()),
                e_over_obbs: EntityOverlappingOBBs::default(),
                target_e_tags: TargetEntityTags(vec![]),
                // e_tag_container: entity_tag_container,
            },
            fns: WeaponFns {
                start_attack: steel_sword_start_attack,
                start_dodge_attack,
                while_attacking: steel_sword_while_attacking,
                after_effect: steel_sword_after_effect,
                end_attack: steel_sword_end_attack,
            },
            state: state_machine(),
            data: WeaponData::new(5, 1000.0, attack_dur, 0.1, 0.05, WeaponState::Owned, WeaponType::SteelSword), 
        },
        test_only: StatusInflictor( DamageOverTime { damage: 2.0, duration_s: Timer::new(0.3), dot_timer: Timer::new(0.1) }),

        // EntityTagContainer(EntityTag::Weapon),
    }).id();

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

pub fn steel_sword_while_attacking(
    ctx: &mut WeaponContext
) {
    ctx.anim_player.play(WeaponAnim::Attack.usize());
    ctx.vel.vec = Vector2::zero();
    // user_vel.vec = user_vel.vec * 0.5;
}
pub fn steel_sword_after_effect(
    ctx: &mut WeaponContext
) {
    // user_vel.vec = user_vel.vec * 0.2;
    ctx.anim_player.stop();
}

// fn(&mut WeaponData, &mut GravityAffected, &mut Velocity, &mut Combat, &mut Sprite, &mut Transform)
pub fn steel_sword_start_attack(
    weapon_ctx: &mut WeaponContext
    // attack_dir: Vector2,
) {
    let attack_dir = weapon_ctx.combat.attack_dir;
    steel_sword_animation(weapon_ctx.sprite, weapon_ctx.trans, attack_dir);
    weapon_ctx.anim_player.play(WeaponAnim::Attack.usize());

    weapon_ctx.weapon_d.knock_dir = attack_dir;
    weapon_ctx.weapon_d.attack_dir = attack_dir;
    // user_vel.vec = attack_dir * 200.0;
    weapon_ctx.vel.vec = Vector2::zero();
    weapon_ctx.grav.0 = false;
    // combat.attacking = true;
}

fn start_dodge_attack(
    ctx: &mut WeaponContext
    // attack_dir: Vector2,
) {
    let attack_dir = ctx.combat.attack_dir;
    steel_sword_animation(ctx.sprite, ctx.trans, attack_dir);
    ctx.anim_player.play(WeaponAnim::Attack.usize());

    ctx.weapon_d.knock_dir = Vector2::new(-attack_dir.x, -attack_dir.y);
    ctx.weapon_d.attack_dir = attack_dir;
    // user_vel.vec = attack_dir * 200.0;
    ctx.vel.vec = Vector2::zero();
    ctx.grav.0 = false;
    // combat.attacking = true;
}

pub fn steel_sword_end_attack(
    ctx: &mut WeaponContext
) {
    // anim_player.stop();
    ctx.vel.vec = Vector2::zero();
    ctx.grav.0 = true;
    // combat.attacking = false;
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

fn state_machine() -> StateMachine<WeaponState> {
    let mut state_m = StateMachine::new(WeaponState::idle());
    state_m.add_state(WeaponState::start_attack());
    state_m.add_state(WeaponState::start_dodge_attack());
    state_m.add_state(WeaponState::start_push_attack());
    state_m.add_state(WeaponState::attacking());
    state_m.add_state(WeaponState::dodge_attacking());
    state_m.add_state(WeaponState::push_attacking());
    state_m.add_state(WeaponState::after_effect_attack());
    state_m.add_state(WeaponState::end_attack());
    state_m
}
