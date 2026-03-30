// pub mod weapon_system;
pub mod steel_sword;
pub mod zombie_arm;

// pub use weapon_system::*;
// pub use steel_sword_system::*;
// pub use zombie_arm_system::*;

use bevy_ecs::prelude::*;
use std::collections::HashMap;

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
        &mut WeaponData,
        &mut Sprite,
        &mut Transform,
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
            &Status,
            Option<&StateMachine<MovementState>>,
        ),
        (With<HeldItem>, Without<HeldBy>
    )>, 
) {
    for (mut weapon_d, mut sprite, mut trans, owned_by, weapon_fns, mut anim_player, mut weapon_state) in &mut query {
        if weapon_state.curr_state() == WeaponState::Unowned {
            continue;
        }

        let owner_entity = owned_by.0;

        if let Ok((mut owner_combat, mut vel, mut grav_affected, mut combat_state, status, move_state)) = owner_query.get_mut(owner_entity) {

            // if owner_combat.stunned() {
                // combat_state.set_state(CombatState::Knocked);
                // weapon_d.temporary_attack_disable();
                // owner_combat.unstun();
            // }
            // else {
            //     combat_state.set_state(CombatState::Idle);
            // }
            match combat_state.curr_state() {
                CombatState::StartAttack => {
                    // println!("hallor...");
                    if weapon_d.can_attack {
                        // println!("this dude was let in...");
                        weapon_d.attack(owner_combat.attack_cd, &mut weapon_state);
                    }
                    
                },
                CombatState::Knocked => { continue; }
                CombatState::KnockEnd => {
                    combat_state.set_state(CombatState::Idle);
                }
                _ => {}
            }

            // if owner_combat.should_attack && weapon_d.can_attack
            // if  combat_state.curr_state() == CombatState::StartAttack && weapon_d.can_attack {
            // } 

            match weapon_state.curr_state() {
                WeaponState::StartAttack => {
                    combat_state.set_state(CombatState::Attacking);
                    weapon_state.set_state(WeaponState::Attacking);

                    let start_attack = weapon_fns.start_attack;
                    start_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans, &mut anim_player);
                },
                WeaponState::Attacking => {
                    let while_attacking = weapon_fns.while_attacking;
                    while_attacking(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans, &mut anim_player);

                },
                WeaponState::AfterEffectAttack => {
                    let after_effect_attack = weapon_fns.after_effect;
                    after_effect_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans, &mut anim_player);
                },
                WeaponState::EndAttack => {
                    // weapon_d.state = WeaponState::Idle;
                    weapon_state.set_state(WeaponState::Idle);
                    combat_state.set_state(CombatState::StopAttacking);
                    let end_attack = weapon_fns.end_attack;
                    end_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans, &mut anim_player);
                },
                _ => {}
            }
        }
    }
}

pub fn attack_timer_and_signal_update(
    mut query: Query<(&mut Sprite, &mut WeaponData, &mut OBB, &mut StateMachine<WeaponState>), With<HeldBy>>,
    delta_time: Res<DeltaTime>
) {
    for (mut sprite, mut weapon_d, mut obb, mut weapon_state) in &mut query {
        if weapon_d.attacking {
            obb.disabled = false;
            sprite.visible = true;
            // anim_player.play(WeaponAnim::Attack.usize());
            weapon_d.attack_timer(delta_time.0, &mut weapon_state);
        }
        else {
            obb.disabled = true;
            sprite.visible = false;
            // cooldown
            if !weapon_d.can_attack {
                weapon_d.attack_cd_timer(delta_time.0, &mut weapon_state);
            }
            if weapon_d.after_effect {
                weapon_d.after_effect_timer(delta_time.0, &mut weapon_state);
            }
        }
    }
}

pub fn lost_owner(mut removed: RemovedComponents<HeldBy>, mut commands: Commands) {
    removed.read().for_each(|e| {
        commands.entity(e).despawn();
    });
}
