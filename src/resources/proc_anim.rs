use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use std::vec::Vec;

#[derive(Resource, Default)]
pub struct ProcAnim {
    pub connections: SparseSet<Entity, Vec<Entity>>
}
