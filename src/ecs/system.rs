use std::collections::HashSet;
use std::any::Any;
use crate::ecs::entity::*;

pub trait System: Any {
    fn entities(&mut self) -> &mut HashSet<Entity>;
    // fn entities_mut(&mut self) -> HashSet<Entity>;
    // fn as_any(&self) -> &dyn Any;
    // fn as_any_mut(&mut self) -> &mut dyn Any;
}

