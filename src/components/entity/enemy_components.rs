use bevy_ecs::prelude::*;

#[derive(Component, Default, Clone)]
pub struct EnemyData {
    pub chase_range: f32,
    pub attack_range: f32,
}
