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

pub fn dist_constraints(
    // mut proc_node: ResMut<ProcAnim>,
    mut query: Query<(
        Entity,
        &mut Transform,
        Option<&mut Constraints>,
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

    for (e, mut trans, constraint_list, mut vel, mut induced) in &mut query {

        // normal clamping
        if let Some(constraint_list) = constraint_list {
            for constraint in &constraint_list.0 {
                let target = constraint.target;
                if let Some(other_trans) = trans_list.get(target)
                {
                    let delta = (other_trans.pos + constraint.target_offset) - trans.pos;
                    let dist = (delta.x*delta.x + delta.y*delta.y).sqrt();
                    let target_pos = other_trans.pos - delta.normalize() * constraint.distance;

                    // // out of bounds, so just fix immediately
                    // if dist > constraint.distance * 5.0 {
                    //     trans.pos = target_pos;
                    //     continue;
                    // }

                    // if let Some(induced) = induced.as_ref()
                    // && let Some(vel) = vel.as_mut()
                    // {
                    //     let induced_vel = (trans.pos - induced.prev_pos) * 2.0;
                    //     vel.vec = induced_vel;
                    // }

                    if dist > constraint.distance {
                        if let Some(vel) = vel.as_mut() 
                        && let Some(induced) = induced.as_mut() {
                            trans.pos = target_pos;
                            vel.vec = Vector2::zero();
                            // let snap_vel = (target_pos - trans.pos) * constraint.stiffness();
                            // vel.vec = induced_vel;
                            // vel.vec = induced_vel;
                            // vel.vec =  snap_vel;
                            induced.prev_pos = trans.pos;
                            // trans.pos = target_pos;
                        }
                        else {
                            // limit to constraint.distance units away from other_trans
                            trans.pos = target_pos;
                        }
                    }
                }
            }
        }
    }
}
