use bevy_ecs::prelude::*;
use std::collections::HashMap;
use rand::*;

use crate::components::*;
use crate::resources::*;

pub fn player_weapon_signal_update(
    mut query: Query<(&PlayerInput, &mut Combat), (With<HeldItem>, Without<HeldBy>)>, 
    mouse_input: Res<MouseInput>,
) {
    for (input, mut combat) in &mut query{
        if input.attack {
            let attack_dir = mouse_input.dir_from_center();
            combat.attack(attack_dir);
        }
        else { combat.not_attack() }
    }
}

pub fn weapon_system_animation_update(
    mut query: Query<(&mut WeaponData, &mut Sprite, &mut Transform, &HeldBy, &WeaponFns, &mut AnimationPlayer)>, 
    mut owner_query: Query<(&mut Combat, &mut Velocity, &mut GravityAffected), (With<HeldItem>, Without<HeldBy>)>, 
) {
    for (mut weapon_d, mut sprite, mut trans, owned_by, weapon_fns, mut anim_player) in &mut query {
        if weapon_d.state == WeaponState::Unowned {
            return;
        }

        let owner_entity = owned_by.0;

        if let Ok((mut owner_combat, mut vel, mut grav_affected)) = owner_query.get_mut(owner_entity) {

            if owner_combat.should_attack && weapon_d.can_attack {
                weapon_d.attack();
            } 

            match weapon_d.state {
                WeaponState::StartAttack => {
                    weapon_d.state = WeaponState::Attacking;
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
                }
                WeaponState::EndAttack => {
                    weapon_d.state = WeaponState::Idle;
                    let end_attack = weapon_fns.end_attack;
                    end_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans, &mut anim_player);
                },
                _ => {}
            }
        }
    }
}

pub fn weapon_attack_timer_and_signal_update(
    mut query: Query<(&mut Sprite, &mut WeaponData, &mut OBB), With<HeldBy>>,
    delta_time: Res<DeltaTime>
) {
    for (mut sprite, mut weapon_d, mut obb) in &mut query {
        if weapon_d.attacking {
            obb.disabled = false;
            sprite.visible = true;
            // anim_player.play(WeaponAnim::Attack.usize());
            weapon_d.attack_timer(delta_time.0);
        }
        else {
            obb.disabled = true;
            sprite.visible = false;
            // cooldown
            if !weapon_d.can_attack {
                weapon_d.attack_cd_timer(delta_time.0);
            }
            if weapon_d.after_effect {
                weapon_d.after_effect_timer(delta_time.0);
            }
        }
    }
}

pub fn weapon_lost_owner(mut removed: RemovedComponents<HeldBy>, mut commands: Commands) {
    removed.read().for_each(|e| {
        commands.entity(e).despawn();
    });
}


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

pub fn entity_hit_update(
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
