use crate::ecs::component::*;
use crate::ecs::entity::*;
use crate::ecs::sparse_set::*;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashSet;
use std::any::*;
use std::collections::HashMap;

// type Type = Vec<TypeId>;
type ArchetypeId = Signature;

// type ComponentArray = Vec<ComponentType>;

#[derive(Default)]
pub struct Archetype {
    // pub signature: Signature,
    // pub ty: Type,
    pub id: ArchetypeId,
    pub components: Vec<UnsafeCell<Box<dyn ISparseSet>>>,
}

// impl Clone for Archetype {
//     fn clone(&self) -> Self {
//         Self {
//             id: self.id.clone(),
//             signature: self.signature.clone(),
//             components: 
//         }
//     }
// }

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
    empty_sets: HashMap<TypeId, Box<dyn ISparseSet>>,
    signatures: HashMap<TypeId, Signature>, // signature is for archetype id's of one component
                                            // type
    new_sign_id: i32,
    // new_arch_id: u16,
    entity_index: HashMap<Entity, Record>,
    component_index: HashMap<TypeId, ArchetypeMap>,
    archetype_map: HashMap<ArchetypeId, Archetype>
    // component_array: HashMap<Signature, Box<dyn ISparseSet>>
}

impl ArchetypeManager {

    fn create_archetype(&mut self, signature: ArchetypeId) -> Archetype {
        let mut components: Vec<UnsafeCell<Box<dyn ISparseSet>>> = Vec::new();
        // loop through signatures map
        for (type_id, bit) in self.signatures.iter() {
            // bitmask check if signature is on archetype id
            if signature & *bit != 0 {
                let column = components.len();

                // get a copy of an empty sparse_list of type from type_id
                let sparse_set = self.empty_sets.get(type_id)
                    .expect("component type not registered")
                    .clone_box();
                components.push(UnsafeCell::new(sparse_set));

                // Register column index
                self.component_index
                    .entry(*type_id)
                    .or_default()
                    .insert(signature, ArchetypeRecord { column });
            }
        }

        // initialize archetype and return
        Archetype {
            id: signature,
            components
        }
    }

    pub fn register_component<T: 'static + Clone>(&mut self) {
        let type_id = TypeId::of::<T>();
        let new_id = 1 << self.new_sign_id;
        self.signatures.insert(type_id, new_id);

        let mut new_arch = Archetype {
            // signature: new_sign,
            id: new_id,
            components: Vec::new(),
        };
        new_arch.components.push(UnsafeCell::new(Box::new(SparseSet::<T>::default())));
        //initialize a copy for the empty sets list
        self.empty_sets.insert(type_id, Box::new(SparseSet::<T>::default()));

        self.archetype_map.insert(new_id, new_arch);
        self.new_sign_id += 1;
        // self.new_arch_id += 1;
    }

    pub fn query_components<T: 'static>(&self) -> impl Iterator<Item = &T> {
        let type_id = TypeId::of::<T>();
        // let arch_map = self.component_index.get(&type_id).expect("component not registered");
        let components: Vec<&T> = vec![];

        components.into_iter()

    }

    pub fn get_component_mut<T: 'static>(&self, entity: Entity) -> Option<&mut T> {
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
                .downcast_mut::<SparseSet<T>>()
                .unwrap()
                .get_mut(record.row)
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        // search for the entity record
        let type_id = TypeId::of::<T>();
        let new_comp_sign = self.signatures.get(&type_id).expect("component not registered");
        
        // if entity is already recorded
        let record = if let Some(record) = self.entity_index.get(&entity) {
            // adding component to an existing entity with component
            unsafe {
                let archetype = &mut *record.archetype;

                // new signature is formed
                let sign = archetype.id;
                // combine prev signature with the added component signature
                let new_id = sign | new_comp_sign;

                let arch_map = if let Some(map) = self.component_index.get(&type_id) {
                    map
                } else {
                    return; 
                };

                // update signature
                // get the column at which the archetype is

                // find the new archetype using the new signature / archetype_id
                if let Some(new_arch) = self.archetype_map.get_mut(&new_id) {
                    // get column to lookup the type immediately
                    let new_arch_rec_column = arch_map.get(&new_arch.id)
                        .unwrap()
                        .column;
                    // insert entity and its components
                    let components = new_arch.components[new_arch_rec_column]
                        .get_mut()
                        .as_any_mut()
                        .downcast_mut::<SparseSet<T>>()
                        .unwrap();

                    // insert entity and its components
                    components.insert(entity, component);
                }
                // if not there, create new archetype with the new signature
                else {
                    let mut new_arch = self.create_archetype(new_id);
                    // get the column
                    let new_arch_rec_column = arch_map.get(&new_arch.id)
                        .unwrap()
                        .column;
                    // insert entity and its components
                    let components = new_arch.components[new_arch_rec_column]
                        .get_mut()
                        .as_any_mut()
                        .downcast_mut::<SparseSet<T>>()
                        .unwrap();

                    // insert entity and its components
                    components.insert(entity, component);

                    self.archetype_map.insert(new_id, new_arch);
                }
                let prev_arch_rec_column = arch_map.get(&archetype.id)
                    .unwrap()
                    .column;
                // take out entity and it's components from prev archetype
                let prev_components = archetype.components[prev_arch_rec_column]
                    .get_mut()
                    .as_any_mut()
                    .downcast_mut::<SparseSet<T>>()
                    .unwrap();


            }
            return;
        };
        // if not in entity index
        // this is the first component of the entity

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
        // find new archetype

    }
}


