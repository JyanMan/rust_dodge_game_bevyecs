use crate::ecs::system::*;
use crate::ecs::entity::*;
use std::collections::HashMap;
use std::any::TypeId;

pub struct SystemManager {
    signatures: HashMap<TypeId, Signature>, 
    systems: HashMap<TypeId, Box<dyn System>>
}

impl SystemManager {
    pub fn register_system<T: 'static + System + Default>(&mut self) -> &mut T {
        let type_id = TypeId::of::<T>();
        let system_rc = Box::new(T::default());
        self.systems.insert(type_id, system_rc);
        // downcast to T
        self.systems.get_mut(&type_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<T>()
            .unwrap()
    }

    pub fn set_signature() {

    }
}


