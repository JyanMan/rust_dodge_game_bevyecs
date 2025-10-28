use bevy_ecs::prelude::*;
use std::vec::Vec;

use crate::ecs::ecs::ECS;
use crate::components::{Transform, Owner, OwnedEntity};

pub fn transform_update_system(mut query: Query<(&mut Transform, &Owner)>, /*parent_query: Query<&Transform, (With<OwnedEntity>, Without<Owner>)>*/) {
    // for (mut pos, owner) in &mut query {
    //     // if (owner)
    //     let owner_pos = parent_query.get(owner.entity).expect("entity has no owner");

    //     pos.global = owner_pos.global + pos.local;
    // }
}
