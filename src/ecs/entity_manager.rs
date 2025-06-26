use std::collections::VecDeque;
use crate::ecs::entity::*;

pub const MAX_ENTITIES: usize = 10;

pub struct EntityManager {
    unused_entities: VecDeque<Entity>,
    entity_signatures: [Signature; MAX_ENTITIES],
    used_entities_count: i32,
}

impl EntityManager {
    pub fn new() -> Self {
        let mut unused_entities = VecDeque::with_capacity(MAX_ENTITIES);
        for entity in 0..MAX_ENTITIES-1  {
            unused_entities.push_back(entity as usize);
        }
        Self {
            unused_entities: unused_entities,
            entity_signatures: [Signature::default(); MAX_ENTITIES],
            used_entities_count: 0,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        assert!(self.used_entities_count < MAX_ENTITIES as i32);

        // take the last entity, and remove it from the unused entities
        let id: Entity = self.unused_entities.pop_front().expect("failed to get entity on creation");
        // the num of used entities increases
        self.used_entities_count += 1; 
        id
    }

    pub fn delete_entity(&mut self, entity: Entity) {
        assert!(self.used_entities_count < MAX_ENTITIES as i32);

        // set the entity signature at index to be empty sign
        self.entity_signatures[entity] = Signature::default();
        // put the deleted entity back to the unused
        self.unused_entities.push_back(entity);
        // decrement used entities
        self.used_entities_count -= 1;
    }

    pub fn set_signature(&mut self, entity: Entity, sign: Signature) {
        assert!(self.used_entities_count < MAX_ENTITIES as i32);
        self.entity_signatures[entity] = sign;
    }
    pub fn get_signature(&self, entity: Entity) {
        self.entity_signatures[entity];
    }
}
