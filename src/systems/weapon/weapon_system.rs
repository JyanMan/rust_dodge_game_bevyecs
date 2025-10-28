use bevy_ecs::prelude::*;
use crate::ecs::ecs::*;
use crate::components::*;
use crate::components::item_handle::*;
use crate::resources::*;

pub fn weapon_system_animation_update(
    mut query: Query<(&mut WeaponData, &mut Sprite, &mut Transform, &HeldBy)>, 
    mut owner_query: Query<&mut Combat, (With<HeldItem>, Without<HeldBy>)>, 
    mouse_input: Res<MouseInput>
) {
    for (mut weapon_d, mut sprite, mut trans, owned_by) in &mut query {
        if weapon_d.state == WeaponState::Unowned {
            return;
        }

        let owner_entity = owned_by.0;

        if let Ok(owner_combat) = owner_query.get_mut(owner_entity) {
            // disallow hold attack button
            if owner_combat.attacking && weapon_d.can_attack {
                weapon_d.attack();
                weapon_d.attack_dir = mouse_input.dir_from_center();
                println!("attacked??");
                weapon_d.w_type.play_anim(&mut sprite, &mut trans, &weapon_d);
            }
            else if !owner_combat.attacking && !weapon_d.attacking {
                weapon_d.can_attack = true;
            }
        }
    }
}

pub fn weapon_attack_timer_and_signal_update(
    mut query: Query<(&mut Sprite, &mut WeaponData, &mut AnimationPlayer), With<HeldBy>>,
    dt_res: Res<DeltaTimeRes>
) {
     for (mut sprite, mut weapon_d, mut anim_player) in &mut query {
         if weapon_d.attacking {
             sprite.visible = true;
             weapon_d.attack_timer(dt_res.delta_time);
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
