use std::collections::HashMap;
use std::any::Any;
use crate::components::sprite::*;
use crate::ecs::entity_manager::*;
use crate::ecs::entity::*;

pub trait ISparseSet: Any {
    fn entity_destroyed(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clone_box(&self) -> Box<dyn ISparseSet>;
    fn move_entity(&mut self, entity: Entity, to: &mut Box<dyn ISparseSet>);
    // fn get(&self, entity: Entity) -> Option<&Any>;
    // fn get_mut(&mut self, entity: Entity) -> Option<&mut Any>; 
    // fn as_slice(&self) -> &[T];
}

impl <T: 'static + Clone> ISparseSet for SparseSet<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn entity_destroyed(&mut self, entity: Entity) {
        self.remove(entity);
    }

    // fn get(&self, entity: Entity) -> Option<&T> {

    // }
    // fn get_mut(&mut self, entity: Entity) -> Option<&mut T>  {

    // }

    fn clone_box(&self) -> Box<dyn ISparseSet> {
        Box::new(self.clone())
    }

    fn move_entity(&mut self, entity: Entity, to: &mut Box<dyn ISparseSet>) {
        self.move_entity(entity, to);
    }
    // fn as_slice(&self) -> &[T] {
    //     self.dense.as_slice()
    // }
}

#[derive(Clone)]
pub struct SparseSet<T> {
    dense: Vec<T>,
    sparse: Vec<Option<usize>>,
    dense_entities: Vec<Entity>
    // data: Vec<Option<T>>,
    // entity_to_index: HashMap<Entity, i32>,
    // index_to_entity: HashMap<i32, Entity>,
    // size: i32,
}

impl <T: 'static> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            dense: Vec::new(),
            sparse: vec![None; MAX_ENTITIES],
            dense_entities: Vec::new(),
        }
    }
}

impl <T: 'static + Clone> SparseSet <T> {
    pub fn move_entity(&mut self, entity: Entity, to: &mut Box<dyn ISparseSet>) {
        if let Some(to_move) = self.get_mut(entity) {
            let sparse_set = to.as_any_mut().downcast_mut::<SparseSet<T>>().unwrap();
            sparse_set.insert(entity, to_move.clone());
            self.remove(entity);
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        self.sparse[entity] = Some(self.dense.len());
        self.dense.push(component);
        self.dense_entities.push(entity);
    }

    pub fn remove(&mut self, entity: Entity) {
        if let Some(curr_index) = self.sparse[entity] {

            let last_index = self.dense.len() - 1;


            //swap dense
            self.dense.swap(curr_index, last_index);

            //swap dense entities
            self.dense_entities.swap(curr_index, last_index);

            // update sparse
            let moved_entity = self.dense_entities[curr_index];
            self.sparse[moved_entity] = Some(curr_index);

            self.dense.pop();
            self.dense_entities.pop();
            self.sparse[entity] = None;
        }
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        if let Some(index) = self.sparse.get(entity) {
            self.dense.get(index.unwrap())
        } 
        else {
            None
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if let Some(index) = self.sparse.get(entity) {
            self.dense.get_mut(index.unwrap())
        } 
        else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
       self.dense_entities
           .iter()
           .zip(self.dense.iter())
           .map(|(&entity, comp)| (entity, comp))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        self.dense_entities
            .iter()
            .cloned()
            .zip(self.dense.iter_mut())
    }

    pub fn len(&self) -> usize {
        self.dense.len()
    }

    pub fn get_by_index(&self, index: usize) -> Option<&T> {
        self.dense.get(index)
    }

    pub fn get_by_index_mut(&self, index: usize) -> Option<&mut T> {
        let ptr = self.dense.as_ptr();
        if index >= self.dense.len() {
            return None
        }
        unsafe {
            Some(&mut *ptr.add(index).cast_mut())
        }
    }
}

