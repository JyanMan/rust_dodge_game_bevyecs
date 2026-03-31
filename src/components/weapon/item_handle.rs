use bevy_ecs::prelude::*;
// use crate::ecs::entity::*;

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Action {
    #[default]
    Idle,
    Use,
    ShiftUse,
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
#[derive(Component, Debug)]
#[relationship_target(relationship = HeldBy)]
pub struct HeldItem {
    #[relationship]
    item: Entity,
    pub action: Action, 
}
impl HeldItem {
    pub fn set_use(&mut self) {
        self.action = Action::Use;
    } 
    pub fn set_shift_use(&mut self) {
        self.action = Action::ShiftUse;
    } 
    pub fn set_idle(&mut self) {
        self.action = Action::Idle;
    } 
    pub fn action(&self) -> Action { self.action.clone() }
}

/// should go together with HeldItem(Entity)
pub struct UsingHeldItem(bool);
