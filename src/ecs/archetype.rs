use crate::ecs::entity::*;
use crate::ecs::sparse_set::*;
use std::any::TypeId;
use std::cell::*;
use std::collections::HashMap;

type ArchetypeId = Signature;

#[derive(Default)]
pub struct Archetype {
    pub id: ArchetypeId,
    pub components: Vec<UnsafeCell<Box<dyn ISparseSet>>>,
    pub component_types: Vec<TypeId>
}

pub(super) struct ArchetypeRecord {
    pub(super) column: usize,
}
type ArchetypeMap = HashMap<ArchetypeId, ArchetypeRecord>;

#[derive(Default)]
pub struct ArchetypeManager {
    pub(super) empty_sets: HashMap<TypeId, Box<dyn ISparseSet>>,
    pub(super) signatures: HashMap<TypeId, Signature>, // signature is for archetype id's of one component
                                                       //
    pub(super) new_sign_id: i32,
    pub(super) entity_index: HashMap<Entity, *mut Archetype>,
    pub(super) component_index: HashMap<TypeId, ArchetypeMap>,
    pub(super) archetype_map: HashMap<ArchetypeId, Archetype>
}

impl ArchetypeManager {
    pub fn create_archetype(&mut self, signature: ArchetypeId) {
        let mut components: Vec<UnsafeCell<Box<dyn ISparseSet>>> = Vec::new();
        let mut component_types: Vec<TypeId> = Vec::new();
        // loop through signatures map
        for (type_id, bit) in self.signatures.iter() {
            // bitmask check if signature is on archetype id
            if signature & *bit == 0 {
                continue;
            }
            // if bit is on signature or archetype_id
            component_types.push(*type_id);

            let column = components.len();

            // get a copy of an empty sparse_list of type from type_id
            let sparse_set = self.empty_sets.get(type_id)
                .expect("component type not registered")
                .clone_box();
            components.push(UnsafeCell::new(sparse_set));

            // Register column index
            if let Some(arch_map) = self.component_index.get_mut(&type_id) {
                assert!(!arch_map.contains_key(&signature));
                arch_map.insert(signature, ArchetypeRecord { column });
            }
            else {
                let new_arch_map = HashMap::from([(signature, ArchetypeRecord { column })]);
                self.component_index.insert(
                    *type_id,
                    new_arch_map
                );
            }
        }

        // initialize archetype and return
        let arch = Archetype {
            id: signature,
            components,
            component_types
        };

        self.archetype_map.insert(signature, arch);
    }

    pub fn register_component<T: 'static + Clone>(&mut self) {
        let type_id = TypeId::of::<T>();
        let new_id = 1 << self.new_sign_id;

        assert!(!self.signatures.contains_key(&type_id));

        self.signatures.insert(type_id, new_id);

        // init empty sparse set for creation of empty of type
        self.empty_sets.insert(type_id, Box::new(SparseSet::<T>::default()));

        // self.create_archetype(new_id);

        //initialize a copy for the empty sets list

        // self.archetype_map.insert(new_id, new_arch);
        self.new_sign_id += 1;
        // self.new_arch_id += 1;
    }

    pub fn get_component_mut<T: 'static + Clone>(&self, entity: Entity) -> Option<&mut T> {
        unsafe {
            let type_id = TypeId::of::<T>();
            let archetype = self.entity_index.get(&entity)
                .expect("entity has no record")
                .as_mut()
                .unwrap();
            // let archetype = record.archetype.as_mut().unwrap();

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
                .get_mut(entity)
        }
    }

    pub fn add_component<T: 'static + Clone>(&mut self, entity: Entity, component: T) {
        // search for the entity record
        let type_id = TypeId::of::<T>();
        let new_comp_sign = *self.signatures.get(&type_id).expect("component not registered");
        
        // if entity is already recorded
        if let Some(arch) = self.entity_index.get(&entity) {
            // adding component to an existing entity with component
            unsafe {
                let prev_arch = &mut **arch;
                // let archetype = &mut *record.archetype;

                // new signature is formed
                let sign = prev_arch.id;
                // combine prev signature with the added component signature
                let new_id = sign | new_comp_sign;

                // update signature
                // get the column at which the archetype is

                // find the new archetype using the new signature / archetype_id
                let new_arch: &mut Archetype = if let Some(new_arch) = self.archetype_map.get_mut(&new_id) {
                    // get column to lookup the type immediately
                    new_arch
                }
                // if not there, create new archetype with the new signature
                else {
                    self.create_archetype(new_id);
                    self.archetype_map.get_mut(&new_id).expect("archetype not created properly")
                };
                // get the archetype map for the new component type
                let arch_map = self.component_index.get(&type_id).expect("component type not registered");

                // get the column
                let new_arch_rec_column = arch_map.get(&new_arch.id)
                    .unwrap()
                    .column;

                // insert entity and its new component to new arch components
                let components = new_arch.components[new_arch_rec_column]
                    .get_mut()
                    .as_any_mut()
                    .downcast_mut::<SparseSet<T>>()
                    .unwrap();
                // insert
                components.insert(entity, component);

                // loop through previous arch
                // move it's components to new arch
                for ty in prev_arch.component_types.iter() {
                    let prev_arch_col = self.component_index[&ty]
                        .get(&prev_arch.id)
                        .expect("archetype is not registered")
                        .column;
                    let new_arch_col = self.component_index[&ty]
                        .get(&new_arch.id)
                        .expect("archetype is not registered")
                        .column;

                    let prev_set = prev_arch.components[prev_arch_col]
                        .get_mut();
                    // get sparse set at new arch at index
                    let new_arch_mut = new_arch.components[new_arch_col]
                        .get()
                        .as_mut()
                        .unwrap();

                    prev_set.move_entity(entity, new_arch_mut);
                }
                self.entity_index.remove(&entity);
                self.entity_index.insert(entity, new_arch);
            }
            return;
        };

        // if not in entity index, this is the first component of the entity
        // get the archetype for solo components
        // NO MORE SOLO COMP ARCH ON REG
        let solo_comp_arch: &mut Archetype = if let Some(new_arch) = self.archetype_map.get_mut(&new_comp_sign) {
            // get column to lookup the type immediately
            new_arch
        }
        // if not there, create new archetype with the new signature
        else {
            self.create_archetype(new_comp_sign);
            self.archetype_map.get_mut(&new_comp_sign).expect("archetype not created properly")
        };
        // let solo_comp_arch = self.archetype_map.get_mut(&new_comp_sign).unwrap();

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

        // insert entity to archetype map
        self.entity_index.insert(entity, solo_comp_arch as *mut _);
        return;
    }
}


