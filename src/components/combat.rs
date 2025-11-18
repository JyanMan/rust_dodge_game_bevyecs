use bevy_ecs::prelude::*;

use crate::components::*;

#[derive(Component, Clone, Default)]
#[component(storage = "Table")]
pub struct Combat {
    pub attacking: bool,
    pub should_attack: bool,
    pub can_attack: bool,
    damage: f32,
    pub attack_dir: Vector2,
    pub attack_cd: f32,
}

impl Combat {
    pub fn new(damage: f32, attack_cd: f32) -> Self {
        Self {
            attacking: false,
            can_attack: true,
            should_attack: false,
            damage,
            attack_dir: Vector2::zero(),
            attack_cd
        }
    }

    pub fn attack(&mut self, attack_dir: Vector2) {
        self.should_attack = true;
        self.attack_dir = attack_dir;
    }
    pub fn not_attack(&mut self) {
        self.should_attack = false;
    }
}
