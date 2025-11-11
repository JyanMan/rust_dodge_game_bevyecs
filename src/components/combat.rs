use bevy_ecs::prelude::*;

#[derive(Component, Clone, Default)]
#[component(storage = "Table")]
pub struct Combat {
    pub attacking: bool,
    pub can_attack: bool,
    damage: f32,
}

impl Combat {
    pub fn new(damage: f32) -> Self {
        Self {
            attacking: false,
            can_attack: true,
            damage,
        }
    }
}
