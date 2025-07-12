use crate::ecs::ecs::*;
use crate::ecs::entity::*;
use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;
use crate::resources::MouseInput;
use std::f64::consts::PI;

pub fn new_steel_sword(ecs: &mut ECS, renderer: &mut Renderer, entity_owner: Entity) {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::SteelSword);
    sprite.set_sprite_sheet(4, 2);
    sprite.visible = true;

    let steel_sword = ecs.spawn::<(Position, Sprite, WeaponData, SteelSwordData, Area, Owner, AnimationPlayer)>((
        Position::zero(),
        sprite,
        WeaponData::new(1, 10.0, 0.2, WeaponState::Owned, WeaponType::SteelSword), 
        SteelSwordData::default(),
        Area::new(0.0, 0.0, 10.0, 10.0),
        Owner::new(entity_owner),
        AnimationPlayer::new(WeaponAnim::COUNT)
    ));

    // init animation
    // do this separately as the pointer to sprite is moved on spawn
    let sprite = ecs.get_component_mut::<Sprite>(steel_sword).unwrap();
    let anim_player = ecs.get_component_mut::<AnimationPlayer>(steel_sword).unwrap();

    let s_frame_ptr = &mut sprite.frame as *mut _;

    let mut idle_anim = Animation::new(1, 0.2);
    idle_anim.set_frame(0, AnimData::Integer { value: 0, target: s_frame_ptr});

    let mut attack_anim = Animation::new(4, 0.05);
    attack_anim.set_frame(0, AnimData::Integer { value: 0, target: s_frame_ptr});
    attack_anim.set_frame(1, AnimData::Integer { value: 1, target: s_frame_ptr});
    attack_anim.set_frame(2, AnimData::Integer { value: 2, target: s_frame_ptr});
    attack_anim.set_frame(3, AnimData::Integer { value: 3, target: s_frame_ptr});

    anim_player.add_anim(WeaponAnim::Idle.usize(), idle_anim);
    anim_player.add_anim(WeaponAnim::Attack.usize(), attack_anim);
}

pub fn steel_sword_animation(ecs: &ECS, sprite: &mut Sprite, e: Entity, owner: &Owner, _delta_time: f32) {

    let mouse_input = ecs.get_resource::<MouseInput>();
    let mouse_dir = mouse_input.dir_from_center();

    // adjust weapon pos
    let owner_pos = ecs.get_component::<Position>(owner.entity).expect("owner has no pos component");
    let self_pos = ecs.get_component_mut::<Position>(e).expect("entity weapon has no pos component");
    // get the local pos based on some magnitude away from parent
    let self_local_pos = mouse_dir * 10.0;
    // adjust pos
    self_pos.vec = owner_pos.vec + self_local_pos;

    let angle_to_mouse = mouse_dir.y.atan2(mouse_dir.x);
    let angle_deg = angle_to_mouse as f64 * (180.0 / PI);

    sprite.angle = angle_deg;
    println!("angle: {}", sprite.angle);
}
