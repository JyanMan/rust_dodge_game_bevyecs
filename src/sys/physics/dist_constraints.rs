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
        // &AttachedTo,
    )>,
    mut trans_list: Local<SparseSet<Entity, Transform>>
) {

    trans_list.clear();
    for (e, trans, _, _) in &query {
        trans_list.insert(e, *trans);
    }

    for (e, mut trans, constraints, vel) in &mut query {

        // println!("trans.pos: {:?}, othertrans: {:?}", other_trans.pos, trans.pos);
        if let Some(constraints) = constraints
        && let Some(target) = constraints.target
        && let Some(other_trans) = trans_list.get(target)
        {
            let delta = other_trans.pos - trans.pos;
            let dist = (delta.x*delta.x + delta.y*delta.y).sqrt();

            if dist > constraints.distance {
                // limit to constraints.distance units away from other_trans
                trans.pos = other_trans.pos - delta.normalize() * constraints.distance;
                if let Some(mut vel) = vel {
                    vel.vec = Vector2::zero();
                }
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
