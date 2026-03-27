use bevy_ecs::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum StatusId {
    Immune,
    DodgeImmune,
    Poisoned
}

#[derive(Component)]
pub struct Status {
    statuses: u32
}

impl Status {
    pub fn new() -> Self {
        Self {
            statuses: 0u32
        }
    }

    pub fn set(&mut self, stat: StatusId) {
        self.statuses |= 1u32 << (stat as u32);
    }
    pub fn unset(&mut self, stat: StatusId) {
        self.statuses &= !(1u32 << (stat as u32));
    }

    pub fn has(&self, stat: StatusId) -> bool {
        self.statuses & (1u32 << (stat as u32)) != 0
    }
}
