use bevy_ecs::prelude::*;
use crate::components::*;
use crate::components::item_handle::*;
use crate::resources::*;

pub fn weapon_system_animation_update(
    mut query: Query<(&mut WeaponData, &mut Sprite, &mut Transform, &HeldBy, &mut OBB)>, 
    mut owner_query: Query<(&mut Combat, &mut Velocity, &mut GravityAffected), (With<HeldItem>, Without<HeldBy>)>, 
    mouse_input: Res<MouseInput>
) {
    for (mut weapon_d, mut sprite, mut trans, owned_by, mut obb) in &mut query {
        if weapon_d.state == WeaponState::Unowned {
            return;
        }

        let owner_entity = owned_by.0;

        if let Ok((owner_combat, mut vel, mut grav_affected)) = owner_query.get_mut(owner_entity) {
            if !owner_combat.attacking && !weapon_d.can_attack {
                weapon_d.w_type.end_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
                obb.disabled = true;
            }
            // disallow hold attack button
            if owner_combat.attacking && weapon_d.can_attack {
                weapon_d.attack();
                weapon_d.attack_dir = mouse_input.dir_from_center();
                weapon_d.w_type.play_anim(&mut sprite, &mut trans, &weapon_d);
                weapon_d.w_type.start_attack_effect(&mut vel, &weapon_d, &mut grav_affected);
                obb.disabled = false;
            }
            else if !owner_combat.attacking && !weapon_d.attacking {
                weapon_d.can_attack = true;
            }
        }
    }
}

pub fn weapon_attack_timer_and_signal_update(
    mut query: Query<(&mut Sprite, &mut WeaponData, &mut AnimationPlayer), With<HeldBy>>,
    delta_time: Res<DeltaTime>
) {
    for (mut sprite, mut weapon_d, mut anim_player) in &mut query {
        if weapon_d.attacking {
            sprite.visible = true;
            weapon_d.attack_timer(delta_time.0);
            anim_player.play(WeaponAnim::Attack.usize());
        }
        else {
            sprite.visible = false;
            anim_player.stop();
        }
    }
}
// so you have a weapon data
    // to which all weapon entities have one
// you also have handle component
    // to which all players and entities that can attack has one
// I query pos sprite and weapondata,
    // check if it has handle, that means, it is owned
    // if it is owned, check if the handle is on state, USING
    // this means weapon is being used to attack
