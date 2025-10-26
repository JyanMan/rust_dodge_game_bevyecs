use bevy_ecs::prelude::*;

#[derive(Component, Clone, Default)]
pub struct Combat {
    pub attacking: bool,
    pub can_attack: bool,
}
