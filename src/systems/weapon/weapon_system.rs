use bevy_ecs::prelude::*;
use crate::components::*;
use crate::components::item_handle::*;
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
    mut query: Query<(&mut WeaponData, &mut Sprite, &mut Transform, &HeldBy, &WeaponFns)>, 
    mut owner_query: Query<(&mut Combat, &mut Velocity, &mut GravityAffected), (With<HeldItem>, Without<HeldBy>)>, 
) {
    for (mut weapon_d, mut sprite, mut trans, owned_by, weapon_fns) in &mut query {
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
                    // weapon_d.attack();
                    weapon_d.attack_dir = owner_combat.attack_dir;
                    // weapon_d.w_type.play_anim(&mut sprite, &mut trans, &weapon_d);
                    // weapon_d.w_type.start_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
                    weapon_d.state = WeaponState::Attacking;
                    // owner_combat.attacking = true;
                    let start_attack = weapon_fns.start_attack;
                    start_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans);
                },
                WeaponState::Attacking => {
                    vel.vec = vel.vec * 0.5;
                    let while_attacking = weapon_fns.while_attacking;
                    while_attacking(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans);
                },
                WeaponState::AfterEffectAttack => {
                    vel.vec = vel.vec * 0.2;
                    let after_effect_attack = weapon_fns.after_effect;
                    after_effect_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans);
                }
                WeaponState::EndAttack => {
                    println!("end attack");
                    // weapon_d.w_type.end_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
                    // owner_combat.attacking = false;
                    weapon_d.state = WeaponState::Idle;
                    let end_attack = weapon_fns.end_attack;
                    end_attack(&mut weapon_d, &mut grav_affected, &mut vel, &mut owner_combat, &mut sprite, &mut trans);
                },
                _ => {}
            }
            
            // if !owner_combat.attacking && !weapon_d.can_attack {
            //     weapon_d.w_type.end_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
            //     println!("asdasdf");
            // }
            // // disallow hold attack button
            // if input.attack && weapon_d.can_attack {
            //     weapon_d.attack();
            //     weapon_d.attack_dir = mouse_input.dir_from_center();
            //     weapon_d.w_type.play_anim(&mut sprite, &mut trans, &weapon_d);
            //     weapon_d.w_type.start_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
            // }
            // else if !owner_combat.attacking && !weapon_d.attacking {
            //     weapon_d.can_attack = true;
            // }
        }
    }
}

// pub fn enemy_weapon_system_animation_update(
//     mut query: Query<(&mut WeaponData, &mut Sprite, &mut Transform, &HeldBy)>, 
//     mut owner_query: Query<(&mut Combat, &mut Velocity, &mut GravityAffected, &EnemyTag), (With<HeldItem>, Without<HeldBy>)>, 
// ) {
//     for (mut weapon_d, mut sprite, mut trans, owned_by) in &mut query {
//         if weapon_d.state == WeaponState::Unowned {
//             return;
//         }

//         let owner_entity = owned_by.0;

//         if let Ok((mut owner_combat, mut vel, mut grav_affected, _)) = owner_query.get_mut(owner_entity) {
//             match weapon_d.state {
//                 WeaponState::Attacking => {
//                     vel.vec = vel.vec * 0.5;
//                 },
//                 WeaponState::EndAttack => {
//                     weapon_d.w_type.end_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
//                     owner_combat.attacking = false;
//                     weapon_d.state = WeaponState::Idle;
//                 },
//                 WeaponState::StartAttack => {
//                     // weapon_d.attack_dir = mouse_input.dir_from_center();
//                     weapon_d.w_type.play_anim(&mut sprite, &mut trans, &weapon_d);
//                     weapon_d.w_type.start_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
//                     weapon_d.state = WeaponState::Attacking;
//                 }
//                 _ => {}
            
//             // if !owner_combat.attacking && !weapon_d.can_attack {
//             //     weapon_d.w_type.end_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
//             // }
//             // // disallow hold attack button
//             // if owner_combat.attacking && weapon_d.can_attack {
//             //     weapon_d.attack();
//             //     weapon_d.attack_dir = owner_combat.attack_dir;
//             //     // weapon_d.attack_dir = mouse_input.dir_from_center();
//             //     weapon_d.w_type.play_anim(&mut sprite, &mut trans, &weapon_d);
//             //     weapon_d.w_type.start_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
//             }
//             // else if !owner_combat.attacking && !weapon_d.attacking {
//             //     // weapon_d.can_attack = true;
//             // }
//         }
//     }
// }

pub fn weapon_attack_timer_and_signal_update(
    mut query: Query<(&mut Sprite, &mut WeaponData, &mut AnimationPlayer, &mut OBB), With<HeldBy>>,
    delta_time: Res<DeltaTime>
) {
    for (mut sprite, mut weapon_d, mut anim_player, mut obb) in &mut query {
        if weapon_d.attacking {
            obb.disabled = false;
            sprite.visible = true;
            anim_player.play(WeaponAnim::Attack.usize());
            weapon_d.attack_timer(delta_time.0);
        }
        else {
            obb.disabled = true;
            sprite.visible = false;
            anim_player.stop();
        }

        // cooldown
        if !weapon_d.attacking {
            if !weapon_d.can_attack {
                weapon_d.attack_cd_timer(delta_time.0);
            }
            if weapon_d.after_effect {
                weapon_d.after_effect_timer(delta_time.0);
            }
        }
    }
}

pub fn weapon_lost_owner(mut removed: RemovedComponents<HeldBy>, weapon_query: Query<&WeaponData>, mut commands: Commands) {
    removed.read().for_each(|e| {
        commands.entity(e).despawn();
    });
}
