use crate::components::*;
use crate::resources::*;
use crate::ecs::ecs::*;
use crate::ecs::entity::Entity;
use crate::systems::weapon::*;

#[derive(Clone, Default)]
pub enum WeaponType {
    #[default]
    SteelSword
}

impl WeaponType {
    pub fn play_anim(&self, sprite: &mut Sprite, trans: &mut Transform, weapon_d: &WeaponData) {
        match self {
            WeaponType::SteelSword => steel_sword_animation(sprite, trans, weapon_d.attack_dir),
        }
    }
}
