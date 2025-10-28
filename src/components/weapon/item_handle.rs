use bevy_ecs::prelude::*;
// use crate::ecs::entity::*;

#[derive(Component, Clone)]
#[relationship(relationship_target = HeldItem)]
pub struct HeldBy(pub Entity);

// impl Owner {
//     pub fn new(entity_owner: Entity) -> Self {
//         Self {
//             entity: entity_owner,
//         }
//     }
// }
// 
#[derive(Component)]
#[relationship_target(relationship = HeldBy)]
pub struct HeldItem(Entity);
