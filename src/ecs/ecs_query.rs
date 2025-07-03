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


