use crate::ecs::ecs::*;
use crate::components::*;

pub fn weapon_fixed_update(ecs: &mut ECS, _time_step: f32) {
    for (_e, pos, weapon_d, area, owner) in 
        ecs.query_comp::<(&mut Position, &mut WeaponData, &mut Area, &Owner)>() 
    {
        if weapon_d.state == WeaponState::Unowned {
            return;
        }

        let owner_entity = owner.entity;
        let owner_pos = ecs.get_component::<Position>(owner_entity).expect("owner has no position component"); 
        let owner_combat = ecs.get_component::<Combat>(owner_entity).expect("owner has no combat component"); 
        pos.vec = owner_pos.vec;

        area.update_pos(pos.vec.x, pos.vec.y);

        // disallow hold attack button
        if owner_combat.attacking && weapon_d.can_attack {
            weapon_d.attack();
        }
        else if !owner_combat.attacking {
            weapon_d.can_attack = true;
        }
    }
}

pub fn weapon_update(ecs: &mut ECS, delta_time: f32) {
    for (_e, sprite, weapon_d, anim_player, _holder) in 
        ecs.query_comp::<(&mut Sprite, &mut WeaponData, &mut AnimationPlayer, &Owner)>() 
    {
        if weapon_d.attacking {
            sprite.visible = true;
            weapon_d.attack_timer(delta_time);
            anim_player.play(WeaponAnim::Attack.usize());
        }
        else {
            sprite.visible = false;
            anim_player.play(WeaponAnim::Idle.usize());
        }
    }
    // weapon_animation_update(ecs, delta_time);
}

pub fn weapon_animation_update(ecs: &mut ECS, _delta_time: f32) {
    for (_e, weapon_d, anim_player) in
        ecs.query_comp::<(&mut WeaponData, &mut AnimationPlayer)>() 
    {
        if weapon_d.attacking {
            anim_player.play(WeaponAnim::Attack.usize());
        }
        else {
            anim_player.play(WeaponAnim::Idle.usize());
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
