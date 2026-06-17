use bevy_ecs::prelude::*;
use crate::components::LocalTransform;

#[derive(Component)]
pub struct DistanceConstraint {
    pub target: Option<Entity>,
    pub distance: f32,
    pub stiffness: f32
}

#[derive(Component)]
#[require(LocalTransform)]
pub struct Anchor(pub Entity);
