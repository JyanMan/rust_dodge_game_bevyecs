use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct DistanceConstraint {
    pub target: Option<Entity>,
    pub distance: f32,
    pub stiffness: f32
}
