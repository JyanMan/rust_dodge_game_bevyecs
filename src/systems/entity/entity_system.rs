use bevy_ecs::prelude::*;
use crate::components::*;
use crate::systems::world::*;
use std::collections::HashMap;

pub fn entity_hit_update(
    mut query: Query<(
        Entity,
        &EntityTagContainer,
        &EntityOverlappingOBBs,
        &mut Health,
        &mut KnockbackTrigger,
        &Transform,
    )>,
    weapon_query: Query<(Entity, &WeaponData)>,
    mut commands: Commands,
) {
    let mut weapon_cache: HashMap<Entity, &WeaponData> = HashMap::new();
    for (e, weapon_d) in &weapon_query {
        weapon_cache.insert(e, weapon_d);
    }
    for (_, tag, e_over_obbs, mut health, mut knock_trigger, trans) in &mut query {
        if health.immune {
            continue;
        }
        // this is matched manually as hit is mainly done by
        // entities with specific tags that are weapon against them
        match tag.0 {
            EntityTag::Zombie => {
                if  let Some((hitter_e, _)) = 
                    e_over_obbs.0.iter().find(|(_, tag)| {
                        matches!(tag, EntityTag::PlayerWeapon)
                    }) 
                {
                    //calc knock_dir
                    if let Some(hitter_wd) = weapon_cache.get(hitter_e) {
                        health.hit_and_immune(hitter_wd.damage);
                        knock_trigger.trigger(hitter_wd.knock_force as i32, hitter_wd.attack_dir);
                        damage_counter_spawn(&mut commands, trans.global, hitter_wd.damage);
                    }
                }
            },
            EntityTag::Player=> {
                if  let Some((hitter_e, _)) = 
                    e_over_obbs.0.iter().find(|(_, tag)| {
                        matches!(tag, EntityTag::EnemyWeapon)
                    }) 
                {
                    //calc knock_dir
                    if let Some(hitter_wd) = weapon_cache.get(hitter_e) {
                        health.hit_and_immune(hitter_wd.damage);
                        knock_trigger.trigger(hitter_wd.knock_force as i32, hitter_wd.attack_dir);
                        damage_counter_spawn(&mut commands, trans.global, hitter_wd.damage);
                    }
                }
            }
            _ => {}
        }

    }
}


pub fn entity_knocked_reaction(
    mut query: Query<(&KnockbackTrigger, &mut Combat)>,
) {
    for (knock, mut combat) in &mut query {
        if knock.knocked {
            combat.stun()
        }
    }
}
