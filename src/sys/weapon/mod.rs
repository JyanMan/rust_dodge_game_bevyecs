pub mod steel_sword;
pub mod zombie_arm;

use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use std::any::TypeId;

use crate::components::*;
use crate::components::states::*;
use crate::resources::*;

#[allow(clippy::type_complexity)]
pub fn anim_state_update(
    mut query: Query<(
        Entity,
        &mut WeaponConfig,
        &mut Sprite,
        &mut Transform,
        &mut LocalTransform,
        &HeldBy,
        &WeaponFns,
        &mut AnimationPlayer,
        &mut StateMachine<WeaponState>,
        &mut OBB,
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
    for (weapon_e, mut weapon_d, mut sprite, mut trans, mut local, owned_by, weapon_fns, mut anim_player, mut weapon_state, mut obb) in &mut query {
        if weapon_state.curr_state() == WeaponState::Unowned {
            continue;
        }

        let owner_entity = owned_by.0;

        if let Ok((mut owner_combat, mut vel, mut grav_affected, mut combat_state, held_item)) = owner_query.get_mut(owner_entity) {

            let mut ctx = WeaponContext {
                self_e: weapon_e,
                commands: &mut commands,
                grav: &mut grav_affected,
                vel: &mut vel,
                sprite: &mut sprite,
                trans: &mut trans,
                local: &mut local,
                anim_player: &mut anim_player,
                combat: &mut owner_combat,
                weapon_d: &mut weapon_d,
                obb: &mut obb,
            };

            match held_item.action {
                Action::Idle => {
                },
                Action::Use => {
                    if ctx.weapon_d.can_attack {
                        combat_state.set_state(CombatState::StartAttack);
                        weapon_state.set_state(WeaponState::StartAttack);
                        ctx.weapon_d.attack(ctx.combat.attack_cd);
                    }
                },
                Action::ShiftUse => {
                    if ctx.weapon_d.can_attack {
                        combat_state.set_state(CombatState::StartAttack);
                        weapon_state.set_state(WeaponState::StartDodgeAttack);
                        ctx.weapon_d.attack(ctx.combat.attack_cd);
                    }
                }
            }


            let curr_state = weapon_state.curr_state();

            // println!("curr weaopn state: {:?}", curr_state);

            match curr_state {
                WeaponState::StartAttack => {
                    combat_state.set_state(CombatState::Attacking);
                    weapon_state.set_state(WeaponState::Attacking);

                    let start_attack = weapon_fns.start_attack;
                    start_attack(&mut ctx);
                    println!("started attack");

                    use std::f32::consts::PI;
                    let ninety_deg = PI / 2.0;
                    let mut rot = local.rot;

                    // if rotated towards left, mirror the heck out of it
                    if weapon_d.attack_dir.x < 0.0 {
                        rot = PI - rot;
                        trans.scale.x = -1.0;
                    }
                    else {
                        trans.scale.x = 1.0;
                    }

                    local.rot = rot;
                },
                WeaponState::StartDodgeAttack => {
                    combat_state.set_state(CombatState::Attacking);
                    weapon_state.set_state(WeaponState::DodgeAttacking);

                    let start_dodge_attack = weapon_fns.start_dodge_attack;
                    start_dodge_attack(&mut ctx);
                },
                WeaponState::Attacking => {
                    let while_attacking = weapon_fns.while_attacking;
                    while_attacking(&mut ctx);

                },
                WeaponState::DodgeAttacking => {
                    let while_dodge_attacking = weapon_fns.while_dodge_attacking;
                    while_dodge_attacking(&mut ctx);

                },
                WeaponState::AfterEffectAttack => {
                    let after_effect = weapon_fns.after_effect;
                    after_effect(&mut ctx);
                },
                WeaponState::AfterEffectDodgeAttack => {
                    let after_dodge_effect= weapon_fns.after_dodge_effect;
                    after_dodge_effect(&mut ctx);
                },
                WeaponState::EndAttack => {
                    // weapon_d.state = WeaponState::Idle;
                    weapon_state.set_state(WeaponState::Idle);
                    println!("end attack");
                    combat_state.set_state(CombatState::StopAttacking);
                    let end_attack = weapon_fns.end_attack;
                    end_attack(&mut ctx);
                },
                WeaponState::EndDodgeAttack => {
                    // weapon_d.state = WeaponState::Idle;
                    weapon_state.set_state(WeaponState::Idle);
                    combat_state.set_state(CombatState::StopAttacking);
                    let end_dodge_attack = weapon_fns.end_dodge_attack;
                    end_dodge_attack(&mut ctx);
                },
                _ => {}
            }
        }
    }
}

pub fn attack_timer_and_signal_update(
    mut query: Query<(&mut WeaponConfig, &mut OBB, &mut StateMachine<WeaponState>), With<HeldBy>>,
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
