use std::collections::HashSet;
use std::any::Any;
use crate::ecs::entity::*;
use sdl2::event::Event;
use crate::ecs::ecs::*;
use crate::core::renderer::*;

pub type StartFn = Box<dyn FnMut(&mut ECS, &mut Renderer)>;
pub type DrawFn = Box<dyn FnMut(&mut ECS, &mut Renderer)>;
pub type UpdateFn = Box<dyn FnMut(&mut ECS, f32)>;
pub type FixedUpdateFn = Box<dyn FnMut(&mut ECS, f32)>;
pub type InputFn = Box<dyn FnMut(&mut ECS, &Event)>;

// pub trait System: Any {
//     fn entities(&mut self) -> &mut HashSet<Entity>;
//     // fn entities_mut(&mut self) -> HashSet<Entity>;
//     // fn as_any(&self) -> &dyn Any;
//     // fn as_any_mut(&mut self) -> &mut dyn Any;
// }

