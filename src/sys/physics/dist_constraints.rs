use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use crate::resources::*;
use crate::components::*;

pub fn dist_constraints(
    // mut proc_node: ResMut<ProcAnim>,
    mut query: Query<(
        Entity,
        &Transform,
        &mut LocalTransform,
        Option<&DistanceConstraint>,
        // &AttachedTo,
    )>,
    mut trans_list: Local<SparseSet<Entity, Transform>>
) {

    trans_list.clear();
    for (e, trans, _, _) in &query {
        trans_list.insert(e, *trans);
    }

    // for (e, trans, mut local, constraints) in &mut query {

    //     if let Some(constraints) = constraints
    //     && let Some(target) = constraints.target
    //     && let Some(other_trans) = trans_list.get(target)
    //     {
    //         let delta = other_trans.pos() - *trans.pos();
    //         let dist = (delta.x*delta.x + delta.y*delta.y).sqrt();

    //         if dist > constraints.distance {
    //             local.pos
    //         }
    //     }  
        
    // }
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
