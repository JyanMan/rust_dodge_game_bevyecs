use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use crate::resources::*;
use crate::components::*;

// pub fn update_list(
//     query: Query<(
//         Option<&BackwardConstraint>
//         Option<&BackwardConstraint>
//     )>
// ) {
    
// }

pub fn dist_constraints<D: DistanceConstraint + Component>(
    // mut proc_node: ResMut<ProcAnim>,
    mut query: Query<(
        Entity,
        &mut Transform,
        Option<&D>,
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

    for (e, mut trans, constraints, mut vel, induced) in &mut query {

        // normal clamping
        if let Some(constraints) = constraints
        && let Some(target) = constraints.target()
        && let Some(other_trans) = trans_list.get(target)
        {
            let delta = (other_trans.pos + constraints.target_offset()) - trans.pos;
            let dist = (delta.x*delta.x + delta.y*delta.y).sqrt();
            let target_pos = other_trans.pos - delta.normalize() * constraints.distance();

            // out of bounds, so just fix immediately
            if dist > constraints.distance() * 5.0 {
                trans.pos = target_pos;
                continue;
            }

            // if let Some(induced) = induced.as_ref()
            // && let Some(vel) = vel.as_mut()
            // {
            //     let induced_vel = (trans.pos - induced.prev_pos) * 2.0;
            //     vel.vec = induced_vel;
            // }

            if dist > constraints.distance() {
                if let Some(mut vel) = vel 
                && let Some(mut induced) = induced {
                    trans.pos = target_pos;
                    vel.vec = Vector2::zero();
                    // let snap_vel = (target_pos - trans.pos) * constraints.stiffness();
                    // vel.vec = induced_vel;
                    // vel.vec = induced_vel;
                    // vel.vec =  snap_vel;
                    induced.prev_pos = trans.pos;
                    // trans.pos = target_pos;
                }
                else {
                    // limit to constraints.distance units away from other_trans
                    trans.pos = target_pos;
                }
            }
        }
    }
}
