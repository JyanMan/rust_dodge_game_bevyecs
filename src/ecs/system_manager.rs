use crate::ecs::system::*;
use crate::ecs::entity::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;

#[derive(Default)]
pub struct SystemManager {
    signatures: HashMap<TypeId, Signature>, 
    systems: HashMap<TypeId, Rc<RefCell<dyn System>>>
}

impl SystemManager {
    pub fn register_system<T: 'static + System + Default>(&mut self) -> Rc<RefCell<T>> {
        let type_id = TypeId::of::<T>();
        let system_rc = Rc::new(RefCell::new(T::default()));
        self.systems.insert(type_id, system_rc.clone());
        // downcast to T
        self.systems.get_mut(&type_id)
            .unwrap();
        system_rc.clone()
            // .as_any_mut()
            // .downcast_mut::<T>()
            // .unwrap();
    }

    pub fn set_signature<T: 'static>(&mut self, signature: Signature) {
        let type_id = TypeId::of::<T>(); 

        if let Some(_sign) = self.signatures.get(&type_id) {
            println!("double insert of signature");
            return;
        }

        self.signatures.insert(type_id, signature);
    }

    pub fn entity_destroyed(&mut self, entity: Entity) {
        for (_type_id, system_box) in self.systems.iter_mut() {
            system_box.borrow_mut().entities().remove(&entity);
        }
    }

    pub fn entity_signature_changed(&mut self, entity: Entity, entity_signature: Signature) {
        for (type_id, system_box) in self.systems.iter_mut() {
            let system_signature = self.signatures.get_mut(&type_id).expect("failed to change signature");
            // let system = system_box.as_any_mut();
            if entity_signature & *system_signature == *system_signature {
                system_box.borrow_mut().entities().insert(entity);
            }
            else {
                system_box.borrow_mut().entities().remove(&entity);
            }
        }
    }

    // pub fn register_system<T: 'static + System + Default>(&mut self) -> &mut T {
    //     let type_id = TypeId::of::<T>();
    //     let system_rc = Box::new(T::default());
    //     self.systems.insert(type_id, system_rc);
    //     // downcast to T
    //     self.systems.get_mut(&type_id)
    //         .unwrap()
    //         .as_any_mut()
    //         .downcast_mut::<T>()
    //         .unwrap()
    // }

    // pub fn set_signature<T: 'static>(&mut self, signature: Signature) {
    //     let type_id = TypeId::of::<T>(); 

    //     if let Some(_sign) = self.signatures.get(&type_id) {
    //         println!("double insert of signature");
    //         return;
    //     }

    //     self.signatures.insert(type_id, signature);
    // }

    // pub fn entity_destroyed(&mut self, entity: Entity) {
    //     for (_type_id, system_box) in self.systems.iter_mut() {
    //         system_box.entities_mut().remove(&entity);
    //     }
    // }

    // pub fn entity_signature_changed(&mut self, entity: Entity, entity_signature: Signature) {
    //     for (type_id, system_box) in self.systems.iter_mut() {
    //         let system_signature = self.signatures.get_mut(&type_id).unwrap();
    //         // let system = system_box.as_any_mut();
    //         if entity_signature & *system_signature == *system_signature {
    //             system_box.entities_mut().insert(entity);
    //         }
    //         else {
    //             system_box.entities_mut().remove(&entity);
    //         }
    //     }
    // }
}


