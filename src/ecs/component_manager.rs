use crate::ecs::component::*;
use crate::ecs::entity::*;
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct ComponentManager {
    component_types: HashMap<TypeId, ComponentType>,
    component_arrays: HashMap<TypeId, Box<dyn IComponentArray>>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    fn get_component_array_mut<T: 'static>(&mut self) -> Option<&mut ComponentArray<T>> {
        let type_id = TypeId::of::<T>();
        self.component_arrays
        .get_mut(&type_id)
        .and_then(|array| array.as_any_mut().downcast_mut::<ComponentArray<T>>())
    }
    fn get_component_array<T: 'static>(&self) -> Option<&ComponentArray<T>> {
        let type_id = TypeId::of::<T>();
        self.component_arrays
        .get(&type_id)
        .and_then(|array| array.as_any().downcast_ref::<ComponentArray<T>>())
    }

    pub fn register_component<T: 'static + Default>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.component_types.insert(type_id, self.next_component_type);
        self.component_arrays.insert(type_id, Box::new(ComponentArray::<T>::default()));
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
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        if let Some(array) = self.get_component_array_mut::<T>() {
            array.remove(entity);
        } 
        else {
            panic!("Component type not registered!");
        }
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.get_component_array::<T>().unwrap().get(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_component_array_mut::<T>().unwrap().get_mut(entity)
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (_type_id, array) in self.component_arrays.iter_mut() {
            array.entity_destroyed(entity);
        }
    }
}
