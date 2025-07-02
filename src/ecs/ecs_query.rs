use sdl2::event::Event;
use sdl2::keyboard::*;
use std::collections::HashSet;
use std::cell::*;
use std::any::*;
use crate::core::renderer::*;
use crate::ecs::ecs::*;
use crate::ecs::entity::*;
use crate::ecs::component::*;
use crate::ecs::system::*;
use crate::ecs::component_manager::*;
use crate::ecs::entity_manager::*;
use crate::ecs::resource_manager::*;

#[macro_export]
macro_rules! query_entities {
    ($ecs: expr, $($type: ty), + ) => {
        $ecs.query_entities(&[
            $(TypeId::of::<$type>()),+
        ])
    }
}

macro_rules! create_query_impl {
    ($($type: ident), +) => {
        impl<'a, $($type),+> Query<'a> for ($(&$type),+)
        where 
            $($type: 'static),+
        {
            type Output = ($(Option<&'a $type>),+);

            fn fetch(ecs: &'a ECS, entity: Entity) -> Self::Output {
                (
                    $(ecs.get_component::<$type>(entity)),+
                )
            }
        }
    }
}

pub trait Query <'a> {
    type Output;
    fn fetch(ecs: &'a ECS, entity: Entity) -> Self::Output;
}

create_query_impl!(A, B);

// impl<'a, A, B> Query<'a> for (&A, &B)
// where 
//     A: 'static,
//     B: 'static,
// {
//     type Output = (Option<&'a A>, Option<&'a B>);
// 
//     fn fetch(ecs: &'a ECS, entity: Entity) -> Self::Output {
//         (
//             ecs.get_component::<A>(entity),
//             ecs.get_component::<B>(entity)
//         )
//     }
// }

pub(crate) use query_entities;

impl ECS {
    pub fn query_entities(&self, component_types: &[TypeId]) -> HashSet<Entity> {
        self.component_m.query_entities(component_types)
    }

    pub fn query_tuple<'a, Q: Query <'a>>(&'a self, e: Entity) -> Q::Output {
        Q::fetch(self, e)
    }
}


