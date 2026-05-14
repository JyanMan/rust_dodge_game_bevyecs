// pub mod weapon_system;
pub mod steel_sword;
pub mod zombie_arm;

// pub use weapon_system::*;
// pub use steel_sword_system::*;
// pub use zombie_arm_system::*;

use bevy_ecs::prelude::*;
use std::collections::HashMap;
use std::any::TypeId;

use crate::components::*;
use crate::components::states::*;
use crate::resources::*;


pub fn per_frame_update(weapon_d: &WeaponData, obb: &mut OBB) {
    let attack_dir = weapon_d.attack_dir;

    let ang_rad = attack_dir.y.atan2(attack_dir.x);

    obb.rotation = ang_rad ;
    obb.rotate_around(Vector2::zero());
    obb.compute_vertices();
}


pub fn anim_state_update(
    mut query: Query<(
        Entity,
        &mut WeaponData,
        &mut Sprite,
        &mut Transform,
        &mut LocalTransform,
        &HeldBy,
        &WeaponFns,
        &mut AnimationPlayer,
        &mut StateMachine<WeaponState>,
    )>, 
    mut owner_query: Query<(
            &mut Combat,
            &mut Velocity,
            &mut GravityAffected,
            &mut StateMachine<CombatState>,
            &HeldItem,
        ),
        Without<HeldBy>
    >, 
    mut commands: Commands
) {
    for (weapon_e, mut weapon_d, mut sprite, mut trans, mut local, owned_by, weapon_fns, mut anim_player, mut weapon_state) in &mut query {
        if weapon_state.curr_state() == WeaponState::Unowned {
            continue;
        }

        let owner_entity = owned_by.0;

        if let Ok((mut owner_combat, mut vel, mut grav_affected, mut combat_state, held_item)) = owner_query.get_mut(owner_entity) {

            match held_item.action {
                Action::Idle => {},
                Action::Use => {
                    if weapon_d.can_attack {
                        combat_state.set_state(CombatState::StartAttack);
                        weapon_state.set_state(WeaponState::StartAttack);
                        weapon_d.attack(owner_combat.attack_cd);
                    }
                },
                Action::ShiftUse => {
                    if weapon_d.can_attack {
                        combat_state.set_state(CombatState::StartAttack);
                        weapon_state.set_state(WeaponState::StartDodgeAttack);
                        weapon_d.attack(owner_combat.attack_cd);
                    }
                }
            }

            let mut weapon_ctx = WeaponContext {
                self_e: weapon_e,
                commands: &mut commands,
                grav: &mut grav_affected,
                vel: &mut vel,
                sprite: &mut sprite,
                trans: &mut trans,
                local: &mut local,
                anim_player: &mut anim_player,
                combat: &mut owner_combat,
                weapon_d: &mut weapon_d
            };

            let curr_state = weapon_state.curr_state();

            // println!("curr weaopn state: {:?}", curr_state);

            match curr_state {
                WeaponState::StartAttack => {
                    combat_state.set_state(CombatState::Attacking);
                    weapon_state.set_state(WeaponState::Attacking);

                    let start_attack = weapon_fns.start_attack;
                    start_attack(&mut weapon_ctx);
                },
                WeaponState::StartDodgeAttack => {
                    combat_state.set_state(CombatState::Attacking);
                    weapon_state.set_state(WeaponState::DodgeAttacking);

                    let start_dodge_attack = weapon_fns.start_dodge_attack;
                    start_dodge_attack(&mut weapon_ctx);
                },
                WeaponState::Attacking => {
                    let while_attacking = weapon_fns.while_attacking;
                    while_attacking(&mut weapon_ctx);

                },
                WeaponState::DodgeAttacking => {
                    let while_dodge_attacking = weapon_fns.while_dodge_attacking;
                    while_dodge_attacking(&mut weapon_ctx);

                },
                WeaponState::AfterEffectAttack => {
                    let after_effect = weapon_fns.after_effect;
                    after_effect(&mut weapon_ctx);
                },
                WeaponState::AfterEffectDodgeAttack => {
                    let after_dodge_effect= weapon_fns.after_dodge_effect;
                    after_dodge_effect(&mut weapon_ctx);
                },
                WeaponState::EndAttack => {
                    // weapon_d.state = WeaponState::Idle;
                    weapon_state.set_state(WeaponState::Idle);
                    combat_state.set_state(CombatState::StopAttacking);
                    let end_attack = weapon_fns.end_attack;
                    end_attack(&mut weapon_ctx);
                },
                WeaponState::EndDodgeAttack => {
                    // weapon_d.state = WeaponState::Idle;
                    weapon_state.set_state(WeaponState::Idle);
                    combat_state.set_state(CombatState::StopAttacking);
                    let end_dodge_attack = weapon_fns.end_dodge_attack;
                    end_dodge_attack(&mut weapon_ctx);
                },
                _ => {}
            }
        }
    }
}

pub fn attack_timer_and_signal_update(
    mut query: Query<(&mut WeaponData, &mut OBB, &mut StateMachine<WeaponState>), With<HeldBy>>,
    dt: Res<DeltaTime>
) {
    for (mut weapon_d, mut obb, mut weapon_state) in &mut query {
        match weapon_state.curr_state() {
            WeaponState::Attacking | WeaponState::DodgeAttacking => {
                obb.disabled = false;
                // sprite.visible = true;
                if weapon_d.attack_timer.tick(dt.0).just_finished() {
                    weapon_state.set_next_state();
                    obb.disabled = true;
                    // sprite.visible = false;
                }
            },
            WeaponState::AfterEffectAttack | WeaponState::AfterEffectDodgeAttack => {
                if weapon_d.after_effect_timer.tick(dt.0).just_finished() {
                    // weapon_state.set_state(WeaponState::EndAttack);
                    // println!("wtf??");
                    weapon_state.set_next_state();
                }
            }
            WeaponState::Idle => {
                obb.disabled = true;
                // sprite.visible = false;
                if !weapon_d.can_attack
                    && weapon_d.attack_cd_timer.tick(dt.0).just_finished() {
                    weapon_d.can_attack = true;
                }
            }
            _ => {}
        }
        // if weapon_d.attacking {
        //     obb.disabled = false;
        //     sprite.visible = true;
        //     // anim_player.play(WeaponAnim::Attack.usize());
        //     weapon_d.attack_timer(delta_time.0, &mut weapon_state);
        // }
        // else {
        //     obb.disabled = true;
        //     sprite.visible = false;
        //     // cooldown
        //     if !weapon_d.can_attack {
        //         weapon_d.attack_cd_timer(delta_time.0, &mut weapon_state);
        //     }
        //     if weapon_d.after_effect {
        //         weapon_d.after_effect_timer(delta_time.0, &mut weapon_state);
        //     }
        // }
    }
}

pub fn lost_owner(mut removed: RemovedComponents<HeldBy>, mut commands: Commands) {
    removed.read().for_each(|e| {
        commands.entity(e).despawn();
    });
}

pub fn newly_owned(
    mut query: Query<(Entity, &WeaponTag, &HeldBy, &mut TargetEntityTags), Added<TargetEntityTags>>,
    allies: Query<&AllyTag>,
    enemies: Query<&EnemyTag>,
    mut commands: Commands
) {
    for (e, _, held_by, mut target_tags) in &mut query {
        let owner_e = held_by.0;
        if allies.contains(owner_e) {
            target_tags.0.push(TypeId::of::<EnemyTag>());
            commands.entity(e).insert(AllyWeaponTag);

        }
        else if enemies.contains(owner_e) {
            target_tags.0.push(TypeId::of::<AllyTag>());
            commands.entity(e).insert(EnemyWeaponTag);
        }
    }
}
