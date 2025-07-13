use crate::ecs::entity::*;

#[derive(Clone, Default)]
pub struct Owner {
    pub entity: Entity,
}

impl Owner {
    pub fn new(entity_owner: Entity) -> Self {
        Self {
            entity: entity_owner,
        }
    }
}

