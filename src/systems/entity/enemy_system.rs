use bevy_ecs::prelude::*;
use rand::*;
use std::collections::HashMap;

use crate::components::*;

pub fn spawn_damage_counter(commands: &mut Commands, start_pos: Vector2, damage: i32) {
    let mut rng = rand::thread_rng(); 
    let text = format!("{}", damage);
    let x_rand = rng.gen_range(-4..4) as f32;
    let y_rand = rng.gen_range((-25)..(-15)) as f32;
    let offset = Vector2::new(x_rand, y_rand);
    let pos = start_pos + offset;
    commands.spawn((
        TextObject::new(text.as_str(), 4, pos, true),
        DamageCounterTimer::new(),
        Transform::new(pos.x, pos.y)
        // TextObject::new(text)        
    ));
}

pub fn enemy_hit_update(
    mut query: Query<(Entity, &EntityTagContainer, &EntityOverlappingOBBs, &mut Health, &mut KnockbackTrigger, &Transform)>,
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
                        spawn_damage_counter(&mut commands, trans.global, hitter_wd.damage);
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
                        spawn_damage_counter(&mut commands, trans.global, hitter_wd.damage);
                    }
                }
            }
            _ => {}
        }

    }
}
