use bevy_ecs::prelude::*;
use std::f64::consts::PI;
use std::any::TypeId;

use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;
use crate::components::states::*;
use crate::bundles::*;

#[derive(Bundle)]
struct ZombieArmBundle {
    sprite: Sprite,
    tag: ZombieArmTag,
    anim_player: AnimationPlayer,
    held: HeldBy,
    parent: AttachedTo,
    weapon: WeaponBundle,
}

pub fn spawn(world: &mut World, entity_owner: Entity) -> Entity {

    let renderer = world.get_non_send_resource::<Renderer>().unwrap();
    let owner_ref = world.entity(entity_owner);
    // let owner_tag = owner_ref.get::<EntityTagContainer>().unwrap();
    // let entity_tag_container = EntityTagContainer(match owner_tag.0 {
    //     EntityTag::Zombie => EntityTag::EnemyWeapon,
    //     EntityTag::Player => EntityTag::PlayerWeapon,
    //     _ => EntityTag::Weapon,
    // });
    
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::ZombieArm);
    sprite.set_sprite_sheet(2, 2);
    sprite.visible = true;

    let attack_dur = 0.2;

    let zombie_arm_e = world.spawn(ZombieArmBundle {
        sprite,
        tag: ZombieArmTag::default(),
        anim_player: AnimationPlayer::new(WeaponAnim::COUNT),
        held: HeldBy(entity_owner),
        parent: AttachedTo(entity_owner),
        weapon: WeaponBundle {
            hitbox: HitboxBundle {
                obb: OBB::new(20.0, 20.0, Vector2::zero(), true),
                cell_pos: CellPos(Vec::new()),
                e_over_obbs: EntityOverlappingOBBs::default(),
                // target_entity_tags,
                target_e_tags: TargetEntityTags(vec![]),
                // e_tag_container: entity_tag_container,
            },
            tag: WeaponTag,
            trans: Transform::zero(),
            fns: WeaponFns {
                start_attack,
                start_dodge_attack,
                while_attacking,
                while_dodge_attacking,
                after_effect,
                after_dodge_effect,
                end_attack,
                end_dodge_attack,
            },
            state: state_machine(),
            // data: WeaponData::new(2, 800.0, attack_dur, 0.4, 0.1, WeaponState::Owned, WeaponType::ZombieArm), 
            data: WeaponConfig {
                damage: 2,
                knock_force: 800.0,
                attacking: false,
                // after_effect: false,
                // attack_dur,
                attack_timer: Timer::new(attack_dur),
                attack_cd_timer: Timer::new(0.1),
                after_effect_timer: Timer::new(0.1),
                can_attack: true,
                attack_dir: Vector2::zero(),
                knock_dir: Vector2::zero(),
                // state: WeaponState::Owned,
                // WeaponType::ZombieArm
            }, 
        },
        // EntityTagContainer(EntityTag::Weapon),
    }).id();

    // init animation
    // do this separately as the pointer to sprite is moved on spawn
    let mut steelsword_ref = world.entity_mut(zombie_arm_e);
    let mut anim_player = steelsword_ref.get_mut::<AnimationPlayer>().unwrap();

    let attack_anim = Animation::new(attack_dur / 8.0, AnimFrames::new(&[
        &[
            AnimData::SpriteFrame { value: 0, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(0.0, -8.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 0, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(4.0, -4.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 1, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(9.0, -2.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 1, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(11.0, 0.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 2, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(11.0, 0.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 2, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(9.0, 2.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 3, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(4.0, 4.0), target: zombie_arm_e},
        ],
        
        &[
            AnimData::SpriteFrame { value: 3, target: zombie_arm_e},
            AnimData::TransformLocal { value: Vector2::new(0.0, 8.0), target: zombie_arm_e},
        ],
    ]));

    anim_player.add_anim(WeaponAnim::Attack.usize(), attack_anim);

    zombie_arm_e
}

pub fn zombie_arm_animation(sprite: &mut Sprite, local: &mut LocalTransform, attack_dir: Vector2) {
    // flip y if left side 
    // this allows animation to be consistent not flipped on another direction
    // if attack_dir.x < 0.0 {
    //     sprite.flip_y = true;
    // }
    // else if attack_dir.x > 0.0 {
    //     sprite.flip_y = false;
    // }

    // convert normalized vec to ang in deg
    let angle_to_mouse = attack_dir.y.atan2(attack_dir.x);
    // let angle_deg = angle_to_mouse as f64 * (180.0 / PI);
    // // adjust sprite angle
    // sprite.angle = angle_deg;

    // let attack_range: f32 = 3.0;
    // local.pos = attack_dir * attack_range;
    local.rot = angle_to_mouse;
}

pub fn while_attacking(
    ctx: &mut WeaponContext
) {
    ctx.anim_player.play(WeaponAnim::Attack.usize());
    ctx.vel.vec = ctx.vel.vec * 0.5;
}
pub fn while_dodge_attacking(
    ctx: &mut WeaponContext
) {
    while_attacking(ctx);
}
pub fn after_effect(
    ctx: &mut WeaponContext
) {
    ctx.vel.vec = ctx.vel.vec * 0.2;
    ctx.anim_player.stop();
}
pub fn after_dodge_effect(
    ctx: &mut WeaponContext
) {
    after_effect(ctx)
}

pub fn start_attack(
    ctx: &mut WeaponContext
) {
    let attack_dir = ctx.combat.attack_dir;
    ctx.sprite.visible = true;
    zombie_arm_animation(ctx.sprite, ctx.local, attack_dir);
    ctx.anim_player.play(WeaponAnim::Attack.usize());


    ctx.weapon_d.knock_dir = attack_dir;
    ctx.weapon_d.attack_dir = attack_dir;
    ctx.vel.vec = attack_dir * 200.0;
    ctx.grav.0 = false;
    // combat.attacking = true;
}
pub fn start_dodge_attack(
    ctx: &mut WeaponContext
) {
    start_attack(ctx)
}

pub fn end_attack(
    ctx: &mut WeaponContext
) {

    ctx.sprite.visible = false;
    // ctx.anim_player.stop();
    ctx.vel.vec = Vector2::zero();
    ctx.grav.0 = true;

    ctx.local.rot = 0.0;
    ctx.local.pos = Vector2::zero();
    // combat.attacking = false;
}

pub fn end_dodge_attack(
    ctx: &mut WeaponContext
) {
    end_attack(ctx);
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

fn state_machine() -> StateMachine<WeaponState> {
    let mut state_m = StateMachine::new(WeaponState::idle());
    state_m.add_state(WeaponState::start_attack());
    state_m.add_state(WeaponState::start_dodge_attack());
    state_m.add_state(WeaponState::start_push_attack());
    state_m.add_state(WeaponState::attacking());
    state_m.add_state(WeaponState::dodge_attacking());
    state_m.add_state(WeaponState::push_attacking());
    state_m.add_state(WeaponState::after_effect_attack());
    state_m.add_state(WeaponState::after_effect_dodge_attack());
    state_m.add_state(WeaponState::after_effect_push_attack());
    state_m.add_state(WeaponState::end_attack());
    state_m.add_state(WeaponState::end_dodge_attack());
    state_m.add_state(WeaponState::end_push_attack());
    state_m
}

fn idle_fn(_ctx: &mut WeaponIdleContext) { }
