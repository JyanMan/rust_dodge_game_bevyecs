use sdl2::event::Event;
use sdl2::keyboard::*;
use std::collections::HashSet;
use std::cell::*;
use std::any::*;
use crate::core::renderer::*;
use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::component_manager::*;
use crate::ecs::entity_manager::*;
use crate::ecs::resource_manager::*;
use crate::ecs::archetype::*;

#[derive(Default)]
pub struct ECS {
    // managers
    pub(super) archetype_m: ArchetypeManager,
    pub(super) component_m: ComponentManager,
    pub(super) entity_m: EntityManager,
    pub(super) resource_m: ResourceManager,

    //systems
    pub(super) startup_systems: Vec<StartFn>,
    pub(super) draw_systems: Vec<DrawFn>,
    pub(super) update_systems: Vec<UpdateFn>,
    pub(super) fixed_update_systems: Vec<FixedUpdateFn>,
    pub(super) input_systems: Vec<InputFn>,

    // ARCHETYPING
    // pub(super) archetypes: Vec<Archetypes>
}

#[allow(warnings)]
impl ECS {
    pub fn new() -> Self {
        Self {
            component_m: ComponentManager::default(),
            archetype_m: ArchetypeManager::default(),
            entity_m: EntityManager::new(),
            resource_m: ResourceManager::new(),

            startup_systems: vec![],
            draw_systems: vec![],
            update_systems: vec![],
            fixed_update_systems: vec![],
            input_systems: vec![],
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entity_m.create_entity()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entity_m.destroy_entity(entity);
        self.component_m.entity_destroyed(entity);
        // self.system_m.entity_destroyed(entity);
    }

    // COMPONENTS

    pub fn register_component<T: 'static + Default + Clone>(&mut self) {
        // self.component_m.register_component::<T>();
        self.archetype_m.register_component::<T>();
    }

    pub fn add_component<T: 'static + Clone>(&mut self, entity: Entity, component: T) {
        self.archetype_m.add_component::<T>(entity, component);
        //self.component_m.add_component::<T>(entity, component);

        //let mut signature = self.entity_m.get_signature(entity);
        //signature |= 1 << self.component_m.get_component_type::<T>().unwrap();
        //
        //self.entity_m.set_signature(entity, signature);
    }

    pub fn remove_component<T: 'static + Clone>(&mut self, entity: Entity) {
        self.component_m.remove_component::<T>(entity);

        let mut signature = self.entity_m.get_signature(entity);
        signature &= !(1 << self.component_m.get_component_type::<T>().unwrap());
        
        self.entity_m.set_signature(entity, signature);
        // self.system_m.entity_signature_changed(entity, signature);
    }

    pub fn get_component<T: 'static + Clone>(&self, entity: Entity) -> Option<&T> {
        self.component_m.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: 'static + Clone>(&self, entity: Entity) -> Option<&mut T> {
        self.component_m.get_component_mut::<T>(entity)
    }

    pub fn get_component_type<T: 'static>(&mut self) -> Option<ComponentType> {
        self.component_m.get_component_type::<T>()
    }

    pub fn has_component<T: 'static + Clone>(&self, entity: Entity) -> bool {
        self.archetype_m.has_component::<T>(entity)
    }

    // SYSTEM CALLS
    pub fn call_startup_systems(&mut self, renderer: &mut Renderer) {
        // Take the list out to avoid mutable borrow overlap
        let mut systems = std::mem::take(&mut self.startup_systems);

        for system in systems.iter_mut() {
            system(self, renderer); // Now we can safely borrow `self` mutably
        }
        // Put them back after running
        self.startup_systems = systems;
    }
    pub fn call_draw_systems(&mut self, renderer: &mut Renderer) {
        let mut systems = std::mem::take(&mut self.draw_systems);

        for system in systems.iter_mut() {
            system(self, renderer); // Now we can safely borrow `self` mutably
        }
        self.draw_systems = systems;
    }
    pub fn call_update_systems(&mut self, delta_time: f32) {
        let mut systems = std::mem::take(&mut self.update_systems);


        for system in systems.iter_mut() {
            system(self, delta_time); // Now we can safely borrow `self` mutably
        }
        // Put them back after running
        self.update_systems = systems;
    }
    pub fn call_fixed_update_systems(&mut self, time_step: f32) {
        let mut systems = std::mem::take(&mut self.fixed_update_systems);

        for system in systems.iter_mut() {
            system(self, time_step); // Now we can safely borrow `self` mutably
        }
        // Put them back after running
        self.fixed_update_systems = systems;
    }
    pub fn call_input_systems(&mut self, k_state: &mut KeyboardState) {
        let mut systems = std::mem::take(&mut self.input_systems);

        for system in systems.iter_mut() {
            system(self, k_state); // Now we can safely borrow `self` mutably
        }
        // Put them back after running
        self.input_systems = systems;
    }

    // SYSTEM REGISTERS

    pub fn register_system_startup(&mut self, system: StartFn) {
        self.startup_systems.push(Box::new(system));
    }

    pub fn register_system_draw(&mut self, system: DrawFn) {
        self.draw_systems.push(Box::new(system));
    }
    pub fn register_system_update(&mut self, system: UpdateFn) {
        self.update_systems.push(Box::new(system));
    }
    pub fn register_system_fixed_update(&mut self, system: FixedUpdateFn) {
        self.fixed_update_systems.push(Box::new(system));
    }
    pub fn register_system_input(&mut self, system: InputFn) {
        self.input_systems.push(Box::new(system));
    }

    // pub fn query_entities(&self, component_types: &[TypeId]) -> HashSet<Entity> {
    //     self.component_m.query_entities(component_types)
    // }

    // RESOURCE
    pub fn add_resource<T: 'static + Any + Default>(&mut self, resource: T) {
        self.resource_m.add_resource::<T>(resource);
    }

    pub fn get_resource_mut<'a, T: 'static + Any + Default>(&'a self) -> RefMut<'a, T> {
        self.resource_m.get_resource_mut::<T>()
    }

    pub fn get_resource<T: 'static + Any + Default>(&self) -> Ref<T> {
        self.resource_m.get_resource::<T>()
    }
}

