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

#[derive(Component, Clone)]
pub struct SteelSwordTag;

#[derive(Component, Clone, Default)]
pub struct ZombieArmTag { }

#[derive(Component, Clone, Default)]
pub struct WeaponTag;

#[derive(Component, Clone, Default)]
pub struct EnemyWeaponTag;

#[derive(Component, Clone, Default)]
pub struct PlayerWeaponTag;
