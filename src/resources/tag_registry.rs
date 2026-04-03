use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use std::collections::HashSet;
use std::vec::Vec;
use std::any::TypeId;

#[derive(Resource, Default)]
pub struct TagRegistry {
    set: SparseSet<Entity, Vec<TypeId>>
}

impl TagRegistry {
    pub fn entity_contains_tag_id(&self, e: Entity, type_id: TypeId) -> bool {
        if let Some(ids) = self.set.get(e)
            && ids.contains(&type_id)
        {
            true
        }
        else {
            false
        }
    }

    pub fn entity_insert<T: 'static>(&mut self, e: Entity) {
        if let Some(ids) = self.set.get_mut(e) {
            ids.push(TypeId::of::<T>());
        }
        else {
            let ids = vec![TypeId::of::<T>()];
            self.set.insert(e, ids);
        }
    }

    pub fn entity_remove<T: 'static>(&mut self, e: Entity) {
        if let Some(ids) = self.set.get_mut(e)
        && let Some(index) = ids.iter().position(|&x| x == TypeId::of::<T>())
        {
            // Use swap_remove for efficiency if order doesn't matter
            ids.swap_remove(index);
        }
    }
}
