use bevy_ecs::prelude::*;
// use crate::ecs::entity::*;

#[derive(Component, Clone)]
#[relationship(relationship_target = OwnedEntity)]
pub struct Owner {
    #[relationship]
    pub entity: Entity,
}

impl Owner {
    pub fn new(entity_owner: Entity) -> Self {
        Self {
            entity: entity_owner,
        }
    }
}

#[derive(Component)]
#[relationship_target(relationship = Owner)]
pub struct OwnedEntity(Entity);
