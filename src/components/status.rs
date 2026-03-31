use bevy_ecs::prelude::*;
use std::vec::Vec;
use strum_macros::EnumDiscriminants;

#[derive(Component)]
pub struct DodgeImmune;

#[derive(Component)]
pub struct DamageOverTime {
    damage: f32,
    time_s: f32,
}
// use bevy_ecs::prelude::*;
// use std::vec::Vec;
// use strum_macros::EnumDiscriminants;

// #[derive(Clone, PartialEq, EnumDiscriminants)]
// pub enum StatusId {
//     None,
//     Immune,
//     DodgeImmune,
//     Poisoned { time_s: f32 },
//     DamageOverTime { damage: f32, time_s: f32 }
// }
// impl StatusId {
//     pub fn usize(&self) -> usize {
//         StatusIdDiscriminants::from(self.clone()) as usize
//     }
// }

// #[derive(Component)]
// pub struct Status {
//     statuses: u32,
//     stat_set: Vec<StatusId>
// }

// impl Status {
//     pub fn new() -> Self {
//         Self {
//             statuses: 0u32,
//             stat_set: Vec::with_capacity(32)
//         }
//     }

//     pub fn set(&mut self, stat: StatusId) {
//         self.stat_set[stat.usize()] = stat.clone();
//         self.statuses |= 1u32 << (stat.usize() as u32);
//     }
//     pub fn unset(&mut self, stat: StatusId) {
//         self.statuses &= !(1u32 << (stat.usize() as u32));
//     }

//     pub fn has(&self, stat: StatusId) -> bool {
//         self.statuses & (1u32 << (stat.usize() as u32)) != 0
//     }

//     pub fn get(&self, stat: StatusId) -> Option<StatusId> {
//        if self.has(stat.clone()) {
//            Some(self.stat_set[stat.usize()].clone())
//        } 
//        else {
//            None
//        }
//     }
// }
