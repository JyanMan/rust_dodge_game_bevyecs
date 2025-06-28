use std::collections::HashMap;
use std::any::Any;
use crate::components::sprite::*;
use crate::ecs::entity_manager::*;
use crate::ecs::entity::*;

pub type ComponentType = u8;

pub trait IComponentArray: Any {
    fn entity_destroyed(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl <T: 'static> IComponentArray for ComponentArray<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn entity_destroyed(&mut self, entity: Entity) {
        self.remove(entity);
    }
}

// #[derive(Default)]
pub struct ComponentArray<T> {
    data: Vec<Option<T>>,
    entity_to_index: HashMap<Entity, i32>,
    index_to_entity: HashMap<i32, Entity>,
    size: i32,
}

impl <T: Clone> Default for ComponentArray<T> {
    fn default() -> Self {
        Self {
            data: vec![None; MAX_ENTITIES],
            entity_to_index: HashMap::new(),
            index_to_entity: HashMap::new(),
            size: 0,
        }
    }
}

impl <T> ComponentArray <T> {
    pub fn insert(&mut self, entity: Entity, component: T) {
        self.data[entity] = Some(component);
        self.entity_to_index.insert(entity, self.size);  
        self.index_to_entity.insert(self.size, entity);
        self.size += 1;
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(&index_of_deleted) = self.entity_to_index.get(&entity) {
            // let index_of_deleted = self.entity_to_index.get(&entity).expect("").clone();
            let index_of_last = self.size - 1;
            if index_of_deleted != index_of_last {
                // Move last element into the deleted spot
                self.data[index_of_deleted as usize] = self.data[index_of_last as usize].take();

                let entity_of_last = self.index_to_entity[&index_of_last];
                self.entity_to_index.insert(entity_of_last, index_of_deleted);
                self.index_to_entity.insert(index_of_deleted, entity_of_last);
            }

            self.entity_to_index.remove(&entity);
            self.index_to_entity.remove(&index_of_last);

            self.size -= 1;
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.data[entity].as_ref()
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.data[entity].as_mut()
    }
}

