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
    pub fn start_attack_effect(&self, vel: &mut Velocity, weapon_d: &WeaponData, grav_affected: &mut GravityAffected) {
        match self {
            WeaponType::SteelSword => steel_sword_start_attack_effect(vel, weapon_d.attack_dir, grav_affected),
        }
    }
    pub fn end_attack_effect(&self, vel: &mut Velocity, weapon_d: &WeaponData, grav_affected: &mut GravityAffected) {
        match self {
            WeaponType::SteelSword => steel_sword_end_attack_effect(vel, grav_affected),
        }
    }
}
