use bevy_ecs::prelude::*;
use std::collections::HashMap;
use bevy_ecs::storage::SparseSet;
use crate::components::*;
use crate::sys;
use crate::sys::world::damage_counter;
use crate::components::states::*;

pub fn update(
    mut enemy_query: Query<(
        // &EntityTagContainer,
        &EnemyTag,
        &EntityOverlappingOBBs,
        &mut Health,
        &mut StateMachine<CombatState>,
        &Transform,
    )>,
    mut player_query: Query<(
        // &EntityTagContainer,
        &PlayerTag,
        &EntityOverlappingOBBs,
        &mut Health,
        &mut StateMachine<CombatState>,
        &Transform,
        Option<&mut DodgeStamina>,
        Option<&DodgeImmune>
    ), Without<EnemyTag>>,
    weapon_query: Query<(Entity, &WeaponData)>,
    mut commands: Commands,
    mut weapon_cache: Local<SparseSet<Entity, WeaponData>>
) {
    for (e, weapon_d) in &weapon_query {
        weapon_cache.insert(e, weapon_d.clone());
    }
    for (_enemy, e_over_obbs, mut health, mut combat_state, trans) in &mut enemy_query {
        if health.immune {
            continue;
        }
        for (hitter_e, _) in e_over_obbs.0.iter() {
            //calc knock_dir
            if let Some(hitter_wd) = weapon_cache.get(*hitter_e) {
                health.hit_and_immune(hitter_wd.damage);

                combat_state.set_state(CombatState::Knocked {
                    dir: hitter_wd.knock_dir,
                    force: hitter_wd.knock_force
                });

                // knock_trigger.trigger(hitter_wd.knock_force as i32, hitter_wd.attack_dir);
                damage_counter::spawn(&mut commands, trans.pos, hitter_wd.damage);
                super::blood_particles::spawn(&mut commands, *trans, hitter_wd.attack_dir);
            }
        }
    }

    for (_player, e_over_obbs, mut health, mut combat_state, trans, mut dodge_stam, dodge_immune) in &mut player_query {
        for (hitter_e, _) in e_over_obbs.0.iter() {
            if health.immune {
                if let Some(dodge_stam) = dodge_stam.as_mut() && dodge_immune.is_some() {
                    dodge_stam.successful_dodge();
                }
                continue;
            }
            //calc knock_dir
            if let Some(hitter_wd) = weapon_cache.get(*hitter_e) {
                health.hit_and_immune(hitter_wd.damage);
                // knock_trigger.trigger(hitter_wd.knock_force as i32, hitter_wd.attack_dir);
                combat_state.set_state(CombatState::Knocked {
                    dir: hitter_wd.knock_dir,
                    force: hitter_wd.knock_force
                });
                damage_counter::spawn(&mut commands, trans.pos, hitter_wd.damage);
            }
        }
    }
}


// pub fn set_knocked_as_stunned(
//     mut query: Query<(&KnockbackTrigger, &mut StateMachine<CombatState>)>,
// ) {
//     for (knock, mut combat_state) in &mut query {
//         if knock.knocked {
//             combat_state.set_state(CombatState::Knocked);
//             // combat.stun()
//         }
//     }
// }
