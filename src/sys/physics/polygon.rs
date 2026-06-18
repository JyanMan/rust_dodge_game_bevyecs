use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
use std::vec::Vec;

use crate::components::*;

// pub fn update_list(
//     mut query: Query<(Entity, &mut PolygonId), Added<PolygonId>>,
//     mut set: ParamSet<(
//         Query<(Entity, &ForwardConstraint)>,
//         Query<(Entity, &BackwardConstraint)>,
//         Query<(Entity, &Anchor)>,
//     )>,
//     mut forward: Local<SparseSet<Entity, ForwardConstraint>>,
//     mut backward: Local<SparseSet<Entity, BackwardConstraint>>,
//     mut anchors: Local<SparseSet<Entity, Anchor>>,
// ) {
    
//     forward.clear();
//     backward.clear();
//     anchors.clear();
//     for (e, constraint) in set.p0() {
//         forward.insert(e, constraint.clone());
//     }
//     for (e, constraint) in set.p1() {
//         backward.insert(e, constraint.clone());
//     }
//     for (e, anchor) in set.p2() {
//         anchors.insert(e, anchor.clone());
//     }
//     let anchor = set.p2();
//     for (e, mut id) in &mut query {
//         for e in id.list.iter() {
//             if let Some(constraint) = forward.get(*e) {
//                 let other_e = constraint.target();
//             }
//             if let Some(constraint) = backward.get(*e) {
//                 let other_e = constraint.target();
//             }
//             else if anchor.contains(*e) {
                
//             }
//         }
//     }
// }
