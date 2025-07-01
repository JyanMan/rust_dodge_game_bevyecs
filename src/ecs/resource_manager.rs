use std::collections::HashMap;
use std::cell::*;
use std::any::*;

#[derive(Default)]
pub struct ResourceManager {
    // using unsafe cell to allow mutable passing on immutable self
    resource_map: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resource_map: HashMap::new()
        }
    }
    pub fn add_resource<T: 'static + Any + Default>(&mut self, resource: T) {
        let type_id = TypeId::of::<T>();
        self.resource_map.insert(type_id, RefCell::new(Box::new(resource)));
    }

    pub fn get_resource<T: 'static + Any + Default>(&self) -> Ref<T> {
        let type_id = TypeId::of::<T>();
        let unsafe_val = self.resource_map.get(&type_id).expect("failed to fetch res from map");
        let borrowed = unsafe_val.borrow();
        let typed_ref = 
            Ref::filter_map(borrowed, |t| {t.downcast_ref::<T>()})
            .ok();

        typed_ref.expect("failed get ref of resource")
    }

    pub fn get_resource_mut<'a, T: 'static + Any + Default>(&'a self) -> RefMut<'a, T> {
        let type_id = TypeId::of::<T>();
        let unsafe_val = self.resource_map.get(&type_id).expect("failed to fetch res from map");
        let borrowed_mut = unsafe_val.borrow_mut();
        let typed_ref_mut = 
            RefMut::map(borrowed_mut, |t| {t.downcast_mut::<T>().expect("mismatched type")});

        typed_ref_mut
    } 
}
