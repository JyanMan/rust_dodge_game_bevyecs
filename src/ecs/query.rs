use crate::ecs::entity::*;
use crate::ecs::ecs::*;

pub trait Query<'a> {
    type Item;
    fn fetch(ecs: &'a ECS, entity: Entity) -> Option<Self::Item>;
}

impl<'a, A, B> Query<'a> for (&'a A, &'a B)
where
    A: 'static,
    B: 'static,
{
    type Item = (&'a A, &'a B);

    fn fetch(ecs: &'a ECS, entity: Entity) -> Option<Self::Item> {
        let a = ecs.get_component::<A>(entity)?;
        let b = ecs.get_component::<B>(entity)?;
        Some((a, b))
    }
}
