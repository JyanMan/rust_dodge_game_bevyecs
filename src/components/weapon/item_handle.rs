use bevy_ecs::prelude::*;
// use crate::ecs::entity::*;

#[derive(Default)]
pub enum ItemType {
    #[default]
    Normal,
    Weapon,
}

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

/// should go together with UsingHeldItem
#[derive(Component)]
#[relationship_target(relationship = HeldBy)]
pub struct HeldItem {
    #[relationship]
    item: Entity,
    e_type: ItemType 
}

/// should go together with HeldItem(Entity)
pub struct UsingHeldItem(bool);
