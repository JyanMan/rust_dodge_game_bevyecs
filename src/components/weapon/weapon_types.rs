use crate::components::{Sprite, Owner};
use crate::ecs::ecs::*;
use crate::ecs::entity::Entity;
use crate::systems::weapon::*;

#[derive(Clone, Default)]
pub enum WeaponType {
    #[default]
    SteelSword
}

impl WeaponType {
    pub fn play_anim(&self, ecs: &ECS, sprite: &mut Sprite, e: Entity, owner: &Owner, delta_time: f32) {
        match self {
            WeaponType::SteelSword => steel_sword_animation(ecs, sprite, e, owner, delta_time),
        }
    }
}
