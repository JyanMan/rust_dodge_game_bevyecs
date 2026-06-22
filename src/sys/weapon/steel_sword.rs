use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use std::f32::consts::PI;

use crate::resources::asset_manager::*;
use crate::components::*;
use crate::components::states::*;
use crate::bundles::*;

#[derive(Bundle)]
struct SteelSwordBundle {
    sprite: Sprite,
    tag: SteelSwordTag,
    anim_player: AnimationPlayer,
    held: HeldBy,
    parent: AttachedTo,
    weapon: WeaponBundle,
    prev_pos: PrevPos,
    // tween: TweenAnim
    // test_only: StatusInflictor<DamageOverTime> 
}

// TODO: HeldBy shouldn't be set on spawn, it should be set dynamically if someone's holding the weapon
pub fn spawn(world: &mut World, entity_owner: Entity) -> Entity {

    let asset_m = world.get_non_send_resource::<AssetManager>().unwrap();
    
    let mut sprite = Sprite::new(&asset_m, TextureId::SteelSword);
    // sprite.set_sprite_sheet(4, 2);
    sprite.visible = true;

    let attack_dur = 0.2;

    let steel_sword_e = world.spawn(SteelSwordBundle {
        // tween: TweenAnim::default(),
        sprite,
        tag: SteelSwordTag,
        prev_pos: PrevPos::default(),
        anim_player: AnimationPlayer::new(WeaponAnim::COUNT),
        held: HeldBy(entity_owner),
        parent: AttachedTo(entity_owner),
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
            // data: WeaponData::new(5, 1000.0, attack_dur, 0.1, 0.05, WeaponState::Owned, WeaponType::SteelSword), 
            data: WeaponConfig {
                damage: 5,
                knock_force: 1000.0,
                attack_timer: Timer::new(attack_dur),
                attack_cd_timer: Timer::new(0.1),
                after_effect_timer: Timer::new(0.05),
                attacking: false,
                can_attack: true,
                attack_dir: Vector2::zero(),
                knock_dir: Vector2::zero(),
                // after_effect: true
                // WeaponState::Owned, WeaponType::SteelSword
            }, 
        },
        // test_only: StatusInflictor( DamageOverTime { damage: 2.0, duration_s: Timer::new(0.3), dot_timer: Timer::new(0.1) }),

        // EntityTagContainer(EntityTag::Weapon),
    }).id();

    // init animation
    // do this separately as the pointer to sprite is moved on spawn
    let mut steelsword_ref = world.entity_mut(steel_sword_e);

    // steelsword_ref.insert(LocalTransform {
    //     origin: Vector2::new(0.0, -20.0),
    //     ..Default::default()
    // });


    let mut anim_player = steelsword_ref.get_mut::<AnimationPlayer>().unwrap();

    let attack_anim = Animation::new(attack_dur / 3.0, AnimFrames::new(&[
        &[
            AnimData::TransformLocal { value: Vector2::new(0.0, -20.0), target: steel_sword_e },
            AnimData::SpriteAngle { value: -50.0, target: steel_sword_e },
        ],
        
        &[
            // AnimData::SpriteFrame { value: 1, target: steel_sword_e},
            // AnimData::TransformLocal { value: Vector2::new(18.0, 0.0), target: steel_sword_e},
            AnimData::TransformLocal { value: Vector2::new(25.0, 0.0), target: steel_sword_e},
            AnimData::SpriteAngle { value: 25.0, target: steel_sword_e },
        ],
       
        &[
            // AnimData::SpriteFrame { value: 3, target: steel_sword_e},
            // AnimData::TransformLocal { value: Vector2::new(0.0, 20.0), target: steel_sword_e},
            AnimData::TransformLocal { value: Vector2::new(0.0, 20.0), target: steel_sword_e},
            AnimData::SpriteAngle { value: 100.0, target: steel_sword_e },
        ],
    ]));

    anim_player.add_anim(WeaponAnim::Attack.usize(), attack_anim);

    steel_sword_e
}

pub fn steel_sword_animation(sprite: &mut Sprite, trans: &mut Transform, local: &mut LocalTransform, attack_dir: Vector2) {
    // flip y if left side 
    // this allows animation to be consistent not flipped on another direction
    // if attack_dir.x < 0.0 {
    //     sprite.flip_y = true;
    // }
    // else if attack_dir.x > 0.0 {
    //     sprite.flip_y = false;
    // }

    // convert normalized vec to ang in deg
    let angle_to_mouse = (attack_dir.y).atan2(attack_dir.x);

    // sprite.angle = 45.0;
    // let angle_deg = angle_to_mouse as f64 * (180.0 / PI);
    // // adjust sprite angle
    // sprite.angle = angle_deg; // to make it point forward


    // let attack_range: f32 = 3.0;
    // local.pos = attack_dir * attack_range;
    local.rot = angle_to_mouse;

}

#[allow(clippy::type_complexity)]
pub fn idle_state(
    mut set: ParamSet<(
        Query<(&mut LocalTransform, &mut Transform, &mut PrevPos, &mut Sprite, &StateMachine<WeaponState>, &HeldBy), With<SteelSwordTag>>,
        Query<(Entity, &Velocity, &Transform)>,
    )>,
    mut user_set: Local<SparseSet<Entity, (Velocity, Transform)>>
) {
    user_set.clear();
    for (e, vel, trans) in &set.p1() {
        user_set.insert(e, (*vel, *trans));
    }

    for (mut local, mut trans, mut prev_pos, mut sprite, state, heldby) in &mut set.p0() {

        if state.curr_state() != WeaponState::Idle {
            prev_pos.pos.clear();
            continue;
        }

        let (user_vel, user_trans) = user_set.get(heldby.0).expect("somehow weapon has user without velocity component");

        if user_vel.vec.x < 0.0 {
            trans.scale.x = -trans.scale.x.abs();
        }
        else if user_vel.vec.x > 0.0 {
            trans.scale.x = trans.scale.x.abs();
        }

        if !prev_pos.pos.is_empty()
        && let Some(actual_prev_pos) = prev_pos.pos.pop_front(){

            const MAX_BEND: f32 = 30.0;

            let pos_delay = actual_prev_pos - user_trans.pos;

            let mut angle_bend = pos_delay * 1.5;

            angle_bend.y = if angle_bend.y.abs() >= MAX_BEND {
                angle_bend.y.signum() * MAX_BEND
            } else {angle_bend.y};

            angle_bend.x = if angle_bend.x.abs() >= MAX_BEND {
                angle_bend.x.signum() * MAX_BEND
            } else {angle_bend.x};

            // use x axis absolute value since weapon is flipped along x axis
            sprite.angle = 50.0 + (angle_bend.x.abs() + angle_bend.y) as f64;
            local.pos = Vector2::new(8.0 - pos_delay.x.abs(), 8.0 + pos_delay.y) ;
            // prev_pos.num_frames_delay = 0;
        } 

        prev_pos.pos.push_back(user_trans.pos); 
        // prev_pos.num_frames_delay += 1;

    }
}

pub fn while_attacking(
    ctx: &mut WeaponContext
) {
    ctx.anim_player.play(WeaponAnim::Attack.usize());
    ctx.vel.vec = Vector2::zero();
}

pub fn after_effect(
    ctx: &mut WeaponContext
) {
    // user_vel.vec = user_vel.vec * 0.2;
    ctx.anim_player.stop();
}

pub fn after_dodge_effect(
    ctx: &mut WeaponContext
) {
    // user_vel.vec = user_vel.vec * 0.2;
    after_effect(ctx);
}

fn start_attack(
    ctx: &mut WeaponContext
    // attack_dir: Vector2,
) {
    let attack_dir = ctx.combat.attack_dir;
    steel_sword_animation(ctx.sprite, ctx.trans, ctx.local, attack_dir);
    ctx.anim_player.play(WeaponAnim::Attack.usize());

    ctx.weapon_d.knock_dir = attack_dir;
    ctx.weapon_d.attack_dir = attack_dir;
    // user_vel.vec = attack_dir * 200.0;
    ctx.vel.vec = Vector2::zero();
    ctx.grav.0 = false;
    // combat.attacking = true;
}

fn start_dodge_attack(
    ctx: &mut WeaponContext
    // attack_dir: Vector2,
) {
    start_attack(ctx);

    let attack_dir = ctx.combat.attack_dir;
    ctx.weapon_d.knock_dir = Vector2::new(-attack_dir.x, -attack_dir.y);

    ctx.commands.entity(ctx.self_e).insert(StatusInflictor(
        DamageOverTime {
            damage: 2.0,
            duration_s: Timer::new(0.2),
            dot_timer: Timer::new(0.05)
        }
    ));
}
fn while_dodge_attacking(ctx: &mut WeaponContext) {
    while_attacking(ctx);

}
fn end_dodge_attack(ctx: &mut WeaponContext) {
    end_attack(ctx);
    ctx.commands.entity(ctx.self_e).remove::<StatusInflictor<DamageOverTime>>();

}

fn end_attack(
    ctx: &mut WeaponContext
) {
    // anim_player.stop();
    ctx.vel.vec = Vector2::zero();
    ctx.grav.0 = true;

    ctx.sprite.angle= 0.0;
    // ctx.sprite.flip_y = false;
    // ctx.sprite.flip_x = false;
    ctx.local.rot = 0.0;
    ctx.local.pos = Vector2::zero();
    // combat.attacking = false;
    // idle_state(ctx);
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
    state_m.add_state(WeaponState::after_effect_dodge_attack());
    state_m.add_state(WeaponState::after_effect_push_attack());
    state_m.add_state(WeaponState::end_attack());
    state_m.add_state(WeaponState::end_dodge_attack());
    state_m.add_state(WeaponState::end_push_attack());
    state_m
}
