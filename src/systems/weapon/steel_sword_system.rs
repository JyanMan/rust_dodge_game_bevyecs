use bevy_ecs::prelude::*;

// use crate::ecs::ecs::*;
// use crate::ecs::entity::*;
use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;
use crate::resources::*;
use crate::resources::MouseInput;
use std::f64::consts::PI;

pub fn steel_sword_spawn(world: &mut World, renderer: &mut Renderer, entity_owner: Entity) -> Entity {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::SteelSword);
    sprite.set_sprite_sheet(4, 2);
    sprite.visible = true;

    let steel_sword_e = world.spawn((
        Transform::zero(),
        sprite,
        WeaponData::new(1, 10.0, 0.2, WeaponState::Owned, WeaponType::SteelSword), 
        SteelSwordTag::default(),
        Area::new(0.0, 0.0, 10.0, 10.0),
        // Owner::new(entity_owner),
        AnimationPlayer::new(WeaponAnim::COUNT),
    )).id();

    // init animation
    // do this separately as the pointer to sprite is moved on spawn
    let mut steelsword_ref = world.entity_mut(steel_sword_e);
    let mut anim_player = steelsword_ref.get_mut::<AnimationPlayer>().unwrap();

    let mut attack_anim = Animation::new(4, 0.05);
    attack_anim.set_frame(0, AnimData::SpriteFrame { value: 0, target: steel_sword_e});
    attack_anim.set_frame(1, AnimData::SpriteFrame { value: 1, target: steel_sword_e});
    attack_anim.set_frame(2, AnimData::SpriteFrame { value: 2, target: steel_sword_e});
    attack_anim.set_frame(3, AnimData::SpriteFrame { value: 3, target: steel_sword_e});

    anim_player.add_anim(WeaponAnim::Attack.usize(), attack_anim);

    steelsword_ref.insert(HeldBy(entity_owner));

    steel_sword_e
}

pub fn steel_sword_animation(sprite: &mut Sprite, trans: &mut Transform, attack_dir: Vector2) {
    // get mouse direction
    // let mouse_input = ecs.get_resource::<MouseInput>();

    // flip y if left side 
    // this allows animation to be consistent not flipped on another direction
    if attack_dir.x < 0.0 {
        sprite.flip_y = true;
    }
    else if attack_dir.x > 0.0 {
        sprite.flip_y = false;
    }

    // convert normalized vec to ang in deg
    let angle_to_mouse = attack_dir.y.atan2(attack_dir.x);
    let angle_deg = angle_to_mouse as f64 * (180.0 / PI);
    // adjust sprite angle
    sprite.angle = angle_deg;

    trans.local = attack_dir * 10.0;
}

pub fn steel_sword_movement_effect(user_vel: &mut Velocity, mouse_input: MouseInput) {
    user_vel.vec = mouse_input.dir_from_center() * 1000.0;
}
