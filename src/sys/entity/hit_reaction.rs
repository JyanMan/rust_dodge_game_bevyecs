use bevy_ecs::prelude::*;
use crate::components::*;
use crate::sys;
use std::collections::HashMap;
use crate::sys::world::damage_counter;

pub fn update(
    mut query: Query<(
        Entity,
        &EntityTagContainer,
        &EntityOverlappingOBBs,
        &mut Health,
        &mut KnockbackTrigger,
        &Transform,
        &mut Status,
        Option<&mut DodgeStamina>
    )>,
    weapon_query: Query<(Entity, &WeaponData)>,
    mut commands: Commands,
) {
    let mut weapon_cache: HashMap<Entity, &WeaponData> = HashMap::new();
    for (e, weapon_d) in &weapon_query {
        weapon_cache.insert(e, weapon_d);
    }
    for (_, tag, e_over_obbs, mut health, mut knock_trigger, trans, status, dodge_stam) in &mut query {
        // this is matched manually as hit is mainly done by
        // entities with specific tags that are weapon against them
        match tag.0 {
            EntityTag::Zombie => {
                if  let Some((hitter_e, _)) = 
                    e_over_obbs.0.iter().find(|(_, tag)| {
                        matches!(tag, EntityTag::PlayerWeapon)
                    }) 
                {
                    if health.immune {
                        continue;
                    }
                    //calc knock_dir
                    if let Some(hitter_wd) = weapon_cache.get(hitter_e) {
                        health.hit_and_immune(hitter_wd.damage);
                        knock_trigger.trigger(hitter_wd.knock_force as i32, hitter_wd.attack_dir);
                        damage_counter::spawn(&mut commands, trans.global, hitter_wd.damage);
                        super::blood_particles::spawn(&mut commands, *trans, hitter_wd.attack_dir);
                    }
                }
            },
            EntityTag::Player=> {
                if let Some((hitter_e, _)) = 
                    e_over_obbs.0.iter().find(|(_, tag)| {
                        matches!(tag, EntityTag::EnemyWeapon)
                    }) 
                {
                    if health.immune {
                        if let Some(mut dodge_stam) = dodge_stam && status.has(StatusId::DodgeImmune)  {
                            dodge_stam.successful_dodge();
                        }
                        continue;
                    }
                    //calc knock_dir
                    if let Some(hitter_wd) = weapon_cache.get(hitter_e) {
                        health.hit_and_immune(hitter_wd.damage);
                        knock_trigger.trigger(hitter_wd.knock_force as i32, hitter_wd.attack_dir);
                        damage_counter::spawn(&mut commands, trans.global, hitter_wd.damage);
                    }
                }
            }
            _ => {}
        }

    }
}


pub fn set_knocked_as_stunned(
    mut query: Query<(&KnockbackTrigger, &mut Combat)>,
) {
    for (knock, mut combat) in &mut query {
        if knock.knocked {
            combat.stun()
        }
    }
}
