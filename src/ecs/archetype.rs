use crate::ecs::component::*;
use crate::ecs::entity::*;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashSet;
use std::any::*;
use std::collections::HashMap;

type Type = Vec<TypeId>;
type ArchetypeId = u16;

// type ComponentArray = Vec<ComponentType>;

pub struct Archetype {
    pub ty: Type,
    pub id: ArchetypeId,
    pub components: Vec<UnsafeCell<Box<dyn IComponentArray>>>,
}

// type ArchetypeSet = HashSet<Archetype>;

struct Record {
    archetype: *mut Archetype,
    row: usize,
}

struct ArchetypeRecord {
    column: usize,
}
type ArchetypeMap = HashMap<ArchetypeId, ArchetypeRecord>;

pub struct ArchetypeManager {
    entity_index: HashMap<Entity, Record>,
    component_index: HashMap<TypeId, ArchetypeMap>
}

impl ArchetypeManager {
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&mut T> {
        unsafe {
            let type_id = TypeId::of::<T>();
            let record = self.entity_index.get(&entity).expect("entity has no record");
            let archetype = record.archetype.as_mut().unwrap();

            let arch_map = if let Some(map) = self.component_index.get(&type_id) {
                map
            } else {
                return None;
            };

            let arch_record = arch_map.get(&archetype.id).unwrap(); 
            archetype.components[arch_record.column]
                .get_mut()
                .as_any_mut()
                .downcast_mut::<T>()
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
                
        // search for the entity record
        let record = if let Some(rec) = self.entity_index.get(&entity) {
            rec
        }
        else {
            //create record
            // if found none, create new record for entity
            // find archetype of entity, if none, create new archetype
            return;
        };
        // find new archetype

    }
}


