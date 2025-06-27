use std::rc::Rc;
use std::cell::RefCell;
use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::component_manager::*;
use crate::ecs::system_manager::*;
use crate::ecs::entity_manager::*;

#[derive(Default)]
pub struct ECS {
    component_m: ComponentManager,
    system_m: SystemManager,
    entity_m: EntityManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            component_m: ComponentManager::default(),
            system_m: SystemManager::default(),
            entity_m: EntityManager::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_m.create_entity()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entity_m.destroy_entity(entity);
        self.component_m.entity_destroyed(entity);
        self.system_m.entity_destroyed(entity);
    }

    pub fn register_component<T: 'static + Default>(&mut self) {
        self.component_m.register_component::<T>();
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_m.add_component::<T>(entity, component);

        let mut signature = self.entity_m.get_signature(entity);
        signature |= 1 << self.component_m.get_component_type::<T>().unwrap();
        
        self.entity_m.set_signature(entity, signature);
        self.system_m.entity_signature_changed(entity, signature);
    }

    pub fn remove_component<T: 'static>(&mut self, entity: Entity) {
        self.component_m.remove_component::<T>(entity);

        let mut signature = self.entity_m.get_signature(entity);
        signature &= !(1 << self.component_m.get_component_type::<T>().unwrap());
        
        self.entity_m.set_signature(entity, signature);
        self.system_m.entity_signature_changed(entity, signature);
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.component_m.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.component_m.get_component_mut::<T>(entity)
    }

    pub fn get_component_type<T: 'static>(&mut self) -> Option<ComponentType> {
        self.component_m.get_component_type::<T>()
    }

    pub fn register_system<T: 'static + System + Default>(&mut self) -> Rc<RefCell<T>> {
        self.system_m.register_system::<T>()
    }

    pub fn set_system_signature<T: 'static>(&mut self, signature: Signature) {
        self.system_m.set_signature::<T>(signature);
    }
}

