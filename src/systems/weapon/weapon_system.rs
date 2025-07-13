use crate::ecs::ecs::*;
use crate::components::*;

pub fn weapon_fixed_update(ecs: &mut ECS, time_step: f32) {
    for (e, weapon_d, owner) in 
        ecs.query_comp::<(&mut WeaponData, &Owner)>() 
    {
        if weapon_d.state == WeaponState::Unowned {
            return;
        }

        let owner_entity = owner.entity;
        // let owner_pos = ecs.get_component::<Position>(owner_entity).expect("owner has no position component"); 
        let owner_combat = ecs.get_component::<Combat>(owner_entity).expect("owner has no combat component"); 

        // disallow hold attack button
        if owner_combat.attacking && weapon_d.can_attack {
            weapon_d.attack();
            weapon_d.w_type.play_anim(ecs, e, owner, time_step);
        }
        else if !owner_combat.attacking && !weapon_d.attacking {
            weapon_d.can_attack = true;
        }
    }
}

pub fn weapon_update(ecs: &mut ECS, delta_time: f32) {
    for (_e, sprite, weapon_d, anim_player, _owner) in 
        ecs.query_comp::<(&mut Sprite, &mut WeaponData, &mut AnimationPlayer, &Owner)>() 
    {
        if weapon_d.attacking {
            sprite.visible = true;
            weapon_d.attack_timer(delta_time);
            anim_player.play(WeaponAnim::Attack.usize());
        }
        else {
            sprite.visible = false;
            anim_player.stop();
        }
    }
    // weapon_animation_update(ecs, delta_time);
}

// pub fn weapon_animation_update(ecs: &mut ECS, delta_time: f32) {
//     for (_e, weapon_d, anim_player, sprite) in
//         ecs.query_comp::<(&mut WeaponData, &mut AnimationPlayer, &mut Sprite)>() 
//     {
//         if weapon_d.attacking {
//             anim_player.play(WeaponAnim::Attack.usize());
//             weapon_d.w_type.play_anim(ecs, sprite, delta_time);
//         }
//         else {
//             anim_player.play(WeaponAnim::Idle.usize());
//         }
//     }
// }

// so you have a weapon data
    // to which all weapon entities have one
// you also have handle component
    // to which all players and entities that can attack has one
// I query pos sprite and weapondata,
    // check if it has handle, that means, it is owned
    // if it is owned, check if the handle is on state, USING
    // this means weapon is being used to attack
