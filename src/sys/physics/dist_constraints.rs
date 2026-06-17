use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use crate::resources::*;
use crate::components::*;

pub fn dist_constraints(
    // mut proc_node: ResMut<ProcAnim>,
    mut query: Query<(
        Entity,
        &mut Transform,
        Option<&DistanceConstraint>,
        Option<&mut Velocity>,
        Option<&mut InducedVelocity>
        // &AttachedTo,
    )>,
    mut trans_list: Local<SparseSet<Entity, Transform>>,
    time_step: Res<TimeStep>
) {

    trans_list.clear();
    for (e, trans, _, _, _) in &query {
        trans_list.insert(e, *trans);
    }

    for (e, mut trans, constraints, vel, induced) in &mut query {

        // normal clamping
        if let Some(constraints) = constraints
        && let Some(target) = constraints.target
        && let Some(other_trans) = trans_list.get(target)
        {
            let delta = other_trans.pos - trans.pos;
            let dist = (delta.x*delta.x + delta.y*delta.y).sqrt();
            let target_pos = other_trans.pos - delta.normalize() * constraints.distance;

            // out of bounds, so just fix immediately
            if dist > constraints.distance * 5.0 {
                trans.pos = target_pos;
                continue;
            }

            if let Some(mut vel) = vel 
            && let Some(mut induced) = induced {
                let induced_vel = (trans.pos - induced.prev_pos) * 2.0;
                let snap_vel = (target_pos - trans.pos) * 80.0;
                vel.vec = snap_vel;
                induced.prev_pos = trans.pos;
            }
            else if dist > constraints.distance {
                // limit to constraints.distance units away from other_trans
                trans.pos = target_pos;
            }
        }
    }
    // for (e, trans, attached_to, constraint) in &query {
    //     let mut e_list = vec![];
    //     let mut prev_e = e;
    //     let mut parent_e = attached_to.0;
    //     loop {
    //         e_list.push(prev_e);
    //         if let Ok((new_e, trans, attached_to ,constraint)) = query.get(parent_e) {
    //             prev_e = new_e;
    //             parent_e = attached_to.0;
    //         }
    //         else {
    //             break;
    //         }
    //     }
    //     proc_node.connections.insert(parent_e, e_list);
    // }
}
