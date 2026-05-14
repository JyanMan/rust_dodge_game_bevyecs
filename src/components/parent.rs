use bevy_ecs::prelude::*;
use crate::components::*;

#[derive(Component)]
#[relationship(relationship_target = Parent)]
#[require(LocalTransform, Transform)]
pub struct AttachedTo(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = AttachedTo)]
pub struct Parent(Entity);
