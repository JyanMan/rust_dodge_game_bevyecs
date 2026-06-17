use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;

use crate::components::*;
use crate::math_helper;

pub fn transform_update(
    mut set: ParamSet<(
        Query<(&mut Transform, &LocalTransform, &AttachedTo, Option<&Anchor>)>, 
        Query<(Entity, &Transform), With<Parent>>
    )>,
    mut parent_trans: Local<SparseSet<Entity, Transform>>
) {
    for (e, trans) in &set.p1() {
        parent_trans.insert(e, *trans);
    }
    for (mut trans, local, attached_to, anchor) in &mut set.p0() {
        // if (owner)

        // use std::f32::consts::PI;
        // let ninety_deg = PI / 2.0;
        // let mut rot = local.rot;
        // let mut temp_offset = local.pos;

        // // if rotated towards left, mirror the heck out of it
        // if local.rot > ninety_deg || local.rot < -ninety_deg {
        //     rot -= PI;
        //     temp_offset.x = -temp_offset.x;
        // }

        // let new_offset = temp_offset.rotate_around(local.origin, rot);
        // local.pos = new_offset;
        let new_offset  = local.pos.rotate_around(local.origin, local.rot);
        let new_offset=  Vector2::new(new_offset.x * trans.scale.x, new_offset.y * trans.scale.y);

        let parent_pos = parent_trans.get(attached_to.0).expect("entity somehow has no parent...");

        trans.pos = parent_pos.pos + new_offset;

        if anchor.is_some() {
            println!("hasdfasdfasdfadsf");
        }

    }
}
