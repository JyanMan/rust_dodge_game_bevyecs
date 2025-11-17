use bevy_ecs::prelude::*;
use std::collections::HashMap;

use crate::components::*;

pub fn enemy_hit_update(
    mut query: Query<(Entity, &EntityTagContainer, &EntityOverlappingOBBs, &mut Health, &mut KnockbackTrigger)>,
    weapon_query: Query<(Entity, &WeaponData)>,
) {
    let mut weapon_cache: HashMap<Entity, &WeaponData> = HashMap::new();
    for (e, weapon_d) in &weapon_query {
        weapon_cache.insert(e, weapon_d);
    }
    for (_, tag, e_over_obbs, mut health, mut knock_trigger) in &mut query {
        if health.immune {
            continue;
        }

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
                    }
                }
            }
            _ => {}
        }

    }
}
