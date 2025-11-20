use bevy_ecs::prelude::*;

#[derive(Component, Clone, Default)]
pub struct PlayerTag {}

#[derive(Component, Clone, Default)]
pub struct ZombieTag {}

#[derive(Component, Clone, Default)]
pub struct EnemyTag {}

#[derive(Component, Clone, Default)]
pub struct HealthBarTag{}

#[derive(Component, Clone, Default)]
pub struct HealthBarFillTag{}

#[derive(Component, Clone, Default)]
pub struct HealthBarTextTag{}

#[derive(Component, Clone, Default)]
pub struct SteelSwordTag { }
