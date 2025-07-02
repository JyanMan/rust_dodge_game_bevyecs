use crate::ecs::component::*;
use crate::ecs::entity::*;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct ComponentManager {
    component_types: HashMap<TypeId, ComponentType>,
    component_arrays: HashMap<TypeId, UnsafeCell<Box<dyn IComponentArray>>>,
    component_entities: HashMap<TypeId, HashSet<Entity>>,
    next_component_type: ComponentType,
    // borrowed_comps: HashSet<u64>
}

impl ComponentManager {
    pub fn query_entities(&self, component_types: &[TypeId]) -> HashSet<Entity> {
        let mut result: Option<HashSet<Entity>> = None;

        // Sort by number of entities with that component (rarer types first)
        let mut sorted_types = component_types.to_vec();
        sorted_types.sort_by_key(|type_id| {
            self.component_entities.get(type_id).map(|set| set.len()).unwrap_or(0)
        });

        for type_id in sorted_types {
            if let Some(entities_with_type) = self.component_entities.get(&type_id) {
                result = match result {
                    None => Some(entities_with_type.clone()),
                    Some(r) => Some(&r & entities_with_type), // set intersection
                };
            } 
            else {
                return HashSet::new(); // No entities have this type
            }
        }
        result.unwrap_or_default()
    }

    fn get_component_array_mut<T: 'static>(&self) -> Option<&mut ComponentArray<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(unsafe_cell) = self.component_arrays.get(&type_id) {
            unsafe {
                let val = unsafe_cell.get().as_mut();
                val.and_then(|array| 
                    array.as_any_mut()
                    .downcast_mut::<ComponentArray<T>>())
            }
        }
        else {
            None
        }
    } 

    fn get_component_array<T: 'static>(&self) -> Option<&ComponentArray<T>> {
        let type_id = TypeId::of::<T>();
        if let Some(unsafe_cell) = self.component_arrays.get(&type_id) {
            unsafe {
                let val = unsafe_cell.get().as_ref();
                val.and_then(|array| 
                    array.as_any()
                    .downcast_ref::<ComponentArray<T>>())
            }
        }
        else {
            None
        }
    } 

    // fn get_component_array_mut_3<A: 'static, B: 'static, C: 'static>(&mut self) 
    //     -> (Option<&mut ComponentArray<A>>,Option<&mut ComponentArray<B>>, Option<&mut ComponentArray<C>>) {
    //     let type_id_a = TypeId::of::<A>();
    //     let type_id_b = TypeId::of::<B>();
    //     let type_id_c = TypeId::of::<C>();

    //     if type_id_a == type_id_b {
    //         return (None, None, None)
    //     }
    //     let three_comps = 
    //         self.component_arrays.get_disjoint_mut([&type_id_a, &type_id_b, &type_id_c]);

    //     if let [Some(a), Some(b), Some(c)] = three_comps {
    //         let casted_a = a.as_any_mut().downcast_mut::<ComponentArray<A>>();
    //         let casted_b = b.as_any_mut().downcast_mut::<ComponentArray<B>>();
    //         let casted_c = c.as_any_mut().downcast_mut::<ComponentArray<C>>();
    //         (casted_a, casted_b, casted_c)
    //     }
    //     else {
    //         (None, None, None)
    //     }
    // }

    // fn get_component_array<T: 'static>(&self) -> Option<&ComponentArray<T>> {
    //     let type_id = TypeId::of::<T>();
    //     self.component_arrays
    //     .get(&type_id).get()
    //     .and_then(|array| array.as_any().downcast_ref::<ComponentArray<T>>())
    // }

    pub fn register_component<T: 'static + Default + Clone>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.component_types.insert(type_id, self.next_component_type);
        self.component_arrays.insert(type_id, UnsafeCell::new(Box::new(ComponentArray::<T>::default())));
    }

    pub fn get_component_type<T: 'static>(&mut self) -> Option<ComponentType> {
        if let Some(&typeid) = self.component_types.get(&TypeId::of::<T>()) {
            Some(typeid)
        }
        else {
            println!("component type not registed");
            None
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        if let Some(array) = self.get_component_array_mut::<T>() {
            array.insert(entity, component);
        } 
        else {
            panic!("Component type not registered!");
        }

        let type_id = TypeId::of::<T>();

        if let Some(entity_set) = self.component_entities.get_mut(&type_id) {
            entity_set.insert(entity);
        }
        else {
            let mut new_set = HashSet::new();
            new_set.insert(entity);
            self.component_entities.insert(type_id, new_set);
        }
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        if let Some(array) = self.get_component_array_mut::<T>() {
            array.remove(entity);
        } 
        else {
            panic!("Component type not registered!");
        }

        let type_id = TypeId::of::<T>();

        if let Some(entity_set) = self.component_entities.get_mut(&type_id) {
            entity_set.remove(&entity);
            if entity_set.is_empty() {
                self.component_entities.remove(&type_id);
            }
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.get_component_array::<T>()?.get(entity)
    }

    pub fn get_component_mut<T: 'static>(&self, entity: Entity) -> Option<&mut T> {
        self.get_component_array_mut::<T>()?.get_mut(entity)
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (_type_id, array) in self.component_arrays.iter_mut() {
            unsafe {
                array.get().as_mut().expect("failed to get array").entity_destroyed(entity);
            }
        }
    }
}
