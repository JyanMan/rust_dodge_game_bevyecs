use crate::ecs::ecs::*;
use crate::ecs::entity::*;
use crate::core::renderer::*;
use crate::resources::asset_manager::*;
use crate::components::*;

pub fn new_steel_sword(ecs: &mut ECS, renderer: &mut Renderer, entity_owner: Entity) {
    let mut sprite = Sprite::new(&renderer.asset_m, TextureId::SteelSword);
    sprite.set_sprite_sheet(4, 2);
    sprite.visible = true;

    let steel_sword = ecs.spawn::<(Position, Sprite, WeaponData, Area, Owner, AnimationPlayer)>((
        Position::zero(),
        sprite,
        WeaponData::new(1, 10.0, 0.2, WeaponState::Owned), 
        Area::new(0.0, 0.0, 10.0, 10.0),
        Owner::new(entity_owner),
        AnimationPlayer::new(4)
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
