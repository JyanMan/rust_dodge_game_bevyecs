use bevy_ecs::prelude::*;

use crate::components::*;

pub fn transform_update_system(
    mut query: Query<(&mut Transform, &HeldBy)>, 
    parent_query: Query<&mut Transform, (With<HeldItem>, Without<HeldBy>)>
) {
    for (mut pos, owned_by) in &mut query {
        // if (owner)
        let owner_pos = parent_query.get(owned_by.0).expect("entity somehow has no owner...");
        pos.global = owner_pos.global + pos.local;
    }
}
