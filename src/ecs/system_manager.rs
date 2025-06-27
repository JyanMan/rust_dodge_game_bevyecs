use crate::ecs::system::*;
use crate::ecs::entity::*;
use crate::ecs::ecs::*;
use crate::core::renderer::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::TypeId;

#[derive(Default)]
pub struct SystemManager {
    signatures: HashMap<TypeId, Signature>, 
    startup_systems: Vec<SystemFn>,
    draw_systems: Vec<SystemFn>
}

impl SystemManager {
    // pub fn register_system_draw<T: 'static + FnMut(&mut ECS, &mut Renderer)>(&mut self, system: T) {
    //     self.draw_systems.push(Box::new(system));
    // }

    pub fn register_system_startup(&mut self, system: SystemFn) {
        self.draw_systems.push(Box::new(system));
    }

    // pub fn register_system_startup<T: 'static + FnMut(&mut ECS, &mut Renderer)>(&mut self, system: T) {
    //     self.startup_systems.push(Box::new(system));
    // }
    pub fn register_system_draw(&mut self, system: SystemFn) {
        self.draw_systems.push(Box::new(system));
    }
    //pub fn register_system<T: 'static + System + Default>(&mut self) -> Rc<RefCell<T>> {
    //    let type_id = TypeId::of::<T>();
    //    let system_rc = Rc::new(RefCell::new(T::default()));
    //    self.systems.insert(type_id, system_rc.clone());
    //    // downcast to T
    //    self.systems.get_mut(&type_id)
    //        .unwrap();
    //    system_rc.clone()
    //        // .as_any_mut()
    //        // .downcast_mut::<T>()
    //        // .unwrap();
    //}

    //pub fn set_signature<T: 'static>(&mut self, signature: Signature) {
    //    let type_id = TypeId::of::<T>(); 

    //    if let Some(_sign) = self.signatures.get(&type_id) {
    //        println!("double insert of signature");
    //        return;
    //    }

    //    self.signatures.insert(type_id, signature);
    //}

    //pub fn entity_destroyed(&mut self, entity: Entity) {
    //    for (_type_id, system_box) in self.systems.iter_mut() {
    //        system_box.borrow_mut().entities().remove(&entity);
    //    }
    //}

    //pub fn entity_signature_changed(&mut self, entity: Entity, entity_signature: Signature) {
    //    for (type_id, system_box) in self.systems.iter_mut() {
    //        let system_signature = self.signatures.get_mut(&type_id).expect("failed to change signature");
    //        // let system = system_box.as_any_mut();
    //        if entity_signature & *system_signature == *system_signature {
    //            system_box.borrow_mut().entities().insert(entity);
    //        }
    //        else {
    //            system_box.borrow_mut().entities().remove(&entity);
    //        }
    //    }
    //}
}


