use std::collections::HashSet;
use std::any::*;
use crate::ecs::ecs::*;
use crate::ecs::entity::*;

#[macro_export]
macro_rules! query_entities {
    ($ecs: expr, $($type: ty), + ) => {
        $ecs.query_entities(&[
            $(TypeId::of::<$type>()),+
        ])
    }
}

macro_rules! create_query_impl {
    ($($($mut:ident)? ($type: ident) ), + $(,)?) => {
        impl<'a, $($type),+> Query<'a> for ($(&$($mut)? $type),+)
        where 
            $($type: 'static),+
        {
            type Output = ($(Option<&'a $($mut)? $type>),+);

            fn fetch(ecs: &'a ECS, entity: Entity) -> Self::Output {
                (
                    $(
                        create_query_impl!(@get $($mut)? ecs, $type, entity)
                    ),+
                )
            }
        }
    };

    // Mutable: expands to ecs.get_component_mut::<T>(entity)
    (@get mut $ecs:ident, $type:ident, $entity:ident) => {
        $ecs.get_component_mut::<$type>($entity)
    };

    // Immutable: expands to ecs.get_component::<T>(entity)
    (@get $ecs:ident, $type:ident, $entity:ident) => {
        $ecs.get_component::<$type>($entity)
    };

}

pub trait Query <'a> {
    type Output;
    fn fetch(ecs: &'a ECS, entity: Entity) -> Self::Output;
}

create_query_impl!((A), (B));
create_query_impl!(mut (A), (B));
create_query_impl!((A), mut (B));
create_query_impl!(mut (A), mut (B));

create_query_impl!((A), (B), (C));
create_query_impl!(mut (A), (B), (C));
create_query_impl!((A), mut (B), (C));
create_query_impl!((A), (B), mut (C));
create_query_impl!((A), mut (B), mut (C));
create_query_impl!(mut (A), mut (B), (C));
create_query_impl!(mut (A), mut (B), mut (C));

create_query_impl!((A), (B), (C), (D));
create_query_impl!(mut (A), (B), (C), (D));
create_query_impl!((A), mut (B), (C), (D));
create_query_impl!((A), (B), mut (C), (D));
create_query_impl!((A), (B), (C), mut (D));
create_query_impl!(mut (A), mut (B), (C), (D));
create_query_impl!(mut (A), (B), mut (C), (D));
create_query_impl!(mut (A), (B), (C), mut (D));
create_query_impl!((A), mut (B), mut (C), (D));
create_query_impl!((A), mut (B), (C), mut (D));
create_query_impl!((A), (B), mut (C), mut (D));
create_query_impl!(mut (A), mut (B), mut (C), (D));
create_query_impl!(mut (A), mut (B), mut (C), mut (D));

pub(crate) use query_entities;

impl ECS {
    pub fn query_entities(&self, component_types: &[TypeId]) -> HashSet<Entity> {
        self.component_m.query_entities(component_types)
    }

    pub fn query_tuple<'a, Q: Query <'a>>(&'a self, e: Entity) -> Q::Output {
        Q::fetch(self, e)
    }
}


