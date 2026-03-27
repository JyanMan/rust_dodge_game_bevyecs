use bevy_ecs::prelude::*;
use bevy_app::prelude::*;

use crate::sys;

pub struct Physics2D;

impl Plugin for Physics2D {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (
            sys::physics::gravity,
            sys::physics::walker_collision .after(sys::physics::gravity),
            sys::physics::pos_vel_update.after(sys::physics::walker_collision),
            sys::physics::transform_update.after(sys::physics::pos_vel_update),
            sys::physics::area_update.after(sys::physics::transform_update),
            sys::physics::obb_update.after(sys::physics::transform_update),
            sys::world::entity_quad::update.after(sys::physics::gravity),
            sys::world::entity_quad::update_overlapping_obbs .after(sys::world::entity_quad::update),
        ));
    }
}
