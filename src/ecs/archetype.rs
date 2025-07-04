use crate::ecs::component::*;
use crate::ecs::entity::*;
use crate::ecs::sparse_set::*;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashSet;
use std::any::*;
use std::collections::HashMap;

// type Type = Vec<TypeId>;
type ArchetypeId = u16;

// type ComponentArray = Vec<ComponentType>;

#[derive(Default)]
pub struct Archetype {
    pub signature: Signature,
    // pub ty: Type,
    pub id: ArchetypeId,
    pub components: Vec<UnsafeCell<Box<dyn ISparseSet>>>,
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
    signatures: HashMap<TypeId, Signature>,
    new_sign_id: i32,
    entity_index: HashMap<Entity, Record>,
    component_index: HashMap<TypeId, ArchetypeMap>,
    archetype_map: HashMap<Signature, Archetype>
    // component_array: HashMap<Signature, Box<dyn ISparseSet>>
}

impl ArchetypeManager {
    pub fn register_component<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        let new_sign = 1 << self.new_sign_id;
        self.signatures.insert(type_id, new_sign);

        let new_arch = Archetype::default();
        self.archetype_map.insert(new_sign, new_arch);
    }

    pub fn query_components<T: 'static>(&self) -> impl Iterator<Item = &T> {
        let type_id = TypeId::of::<T>();
        // let arch_map = self.component_index.get(&type_id).expect("component not registered");
        let components: Vec<&T> = vec![];

        components.into_iter()

    }

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

    // fn get_signature

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        // search for the entity record
        let type_id = TypeId::of::<T>();
        let signature = self.signatures.get(&type_id).expect("component not registered");
        
        let record = if let Some(rec) = self.entity_index.get(&entity) {
            // adding component to an existing entity with component
            let record = self.entity_index.get(&entity).unwrap();
            unsafe {
                let archetype = &mut *record.archetype;

                // new signature is formed
                let sign = archetype.signature;
                let new_sign = sign | signature;

                // update signature
                // search for signature on archetype_map
                    // insert entity and its components there
                // if not there, insert new signature with new archetype
                // take out entity and it's components from prev archetype

            }
        }
        else {
            // addition of the first component of entity

            // get the archetype for solo components
            let solo_comp_arch = self.archetype_map.get_mut(&signature).unwrap();

            // get component index
            let arch_rec = self.component_index.get(&type_id).expect("component not registered / properly");
            let column = arch_rec.get(&solo_comp_arch.id).unwrap().column;

            // insert entity to components
            let components = solo_comp_arch.components.get_mut(column)
                .expect("invalid index")
                .get_mut()
                .as_any_mut()
                .downcast_mut::<SparseSet<T>>()
                .unwrap();

            components.insert(entity, component);

            // initialize record
            let new_rec = Record {
                archetype: solo_comp_arch as *mut _,
                row: entity,
            };

            // insert record
            self.entity_index.insert(entity, new_rec);
            return;
        };
        // find new archetype

    }
}


