use crate::ecs::entity::*;

#[derive(Clone, Default)]
pub struct Owner {
    // pub state: WeaponState,
    pub entity: Entity,
    // pub using: bool,
    // pub pos: Vector2
}

impl Owner {
    pub fn new(entity_owner: Entity) -> Self {
        Self {
            // state: WeaponState::Owned,
            entity: entity_owner,
            // using: false,
            // pos: Vector2::zero(),
        }
    }
}

#[derive(Default, Clone)]
pub enum ItemType {
    #[default]
    Weapon,
}
