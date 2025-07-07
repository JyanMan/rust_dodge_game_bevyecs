use crate::ecs::entity::*;
use crate::ecs::ecs::*;
use crate::ecs::archetype::*;
use crate::ecs::sparse_set::*;
use std::any::*;
use paste::*;

pub trait Bundle {
    type Item;
    fn spawn(am: &mut ArchetypeManager, entity: Entity, components: Self::Item);
}

macro_rules! bundle_impl {
    ($( ($num: tt, $type: ident)), + $(,)? ) => {
        paste! {
            #[allow(non_snake_case)]
            impl<'a, $($type),+> Bundle for ($($type),+) 
            where 
                $($type: 'static + Clone), +
                // B: 'static + Clone
            {
                type Item = ($($type),+);
                fn spawn(am: &mut ArchetypeManager, entity: Entity, components: Self::Item) {
                    // search for the entity record
                    $(let [<type_$type>] = TypeId::of::<$type>();)+
                    $(let [<new_comp_sign_$type>] = am.signatures.get(&[<type_$type>])
                        .expect("component not registered");)+
                    let combined_sign = $(*[<new_comp_sign_$type>])|+; // | other types |
                    
                    // if not in entity index, this is the first component of the entity
                    // get the archetype for solo components
                    let new_arch: &mut Archetype = 
                        if let Some(new_arch) = am.archetype_map.get_mut(&combined_sign) {
                        // get column to lookup the type immediately
                        new_arch
                    }
                    // if not there, create new archetype with the new signature
                    else {
                        am.create_archetype(combined_sign);
                        am.archetype_map.get_mut(&combined_sign).expect("archetype not created properly")
                    };

                    // get component index
                    $(let [<arch_rec_$type>] = am.component_index.get(&[<type_$type>])
                        .expect("component not registered / properly");)+
                    $(let [<column_$type>] = [<arch_rec_$type>].get(&new_arch.id).unwrap().column;)+

                    // insert entity to components
                    unsafe {
                        $(let [<components_$type>] = new_arch.components[[<column_$type>]]
                            .get()
                            .as_mut()
                            .unwrap()
                            .as_any_mut()
                            .downcast_mut::<SparseSet<$type>>()
                            .unwrap();)+

                        $([<components_$type>].insert(entity, components.$num);)+

                        // insert entity to archetype map
                        am.entity_index.insert(entity, new_arch as *mut _);
                    }
                    return;
                }
            }
        }
    }
}

bundle_impl!((0, A), (1, B));
bundle_impl!((0, A), (1, B), (2, C));
bundle_impl!((0, A), (1, B), (2, C), (3, D));
bundle_impl!((0, A), (1, B), (2, C), (3, D), (4, E));
bundle_impl!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F));
bundle_impl!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G));
bundle_impl!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H));
bundle_impl!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I));
bundle_impl!((0, A), (1, B), (2, C), (3, D), (4, E), (5, F), (6, G), (7, H), (8, I), (9, J));

impl ArchetypeManager {
    pub fn spawn<B: Bundle>(&mut self, entity: Entity, components: B::Item) {
        B::spawn(self, entity, components);
    }
}

impl ECS {
    pub fn spawn<B: Bundle>(&mut self, components: B::Item) {
        let entity = self.create_entity();
        self.archetype_m.spawn::<B>(entity, components);
    }
}
