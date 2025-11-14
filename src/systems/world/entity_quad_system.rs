use bevy_ecs::prelude::*;
use std::collections::HashMap;
use crate::components::*;
use crate::resources::*;

/* PHYSICS */
pub fn update_entity_quad_system(
    mut e_quad_map: ResMut<EntityQuadMap>,
    mut query:  Query<(Entity, &mut CellPos, &Transform, &OBB)>
) {
    for (e, mut cell_pos, trans, obb) in &mut query {
        e_quad_map.update_entity_cell(e, *trans, &mut cell_pos, obb);
    }
}

pub fn quad_generation_system(
    mut e_quad_map: ResMut<EntityQuadMap>, 
    query: Query<&Transform, With<PlayerTag>>
) {
    for transform in &query {
        e_quad_map.generate(&transform.global);
    }
}

// pub fn update_entity_overlapping_obbs(
//     mut e_cells_query: Query<(Entity, &mut OBB, &CellPos, &mut EntityOverlappingOBBs, &EntityTagContainer, &TargetEntityTags)>,
//     e_quad_map: ResMut<EntityQuadMap>,
//     mut tmp_entity_map: Local<HashMap<Entity, Vec<Entity>>>,
// ) {
//     tmp_entity_map.clear();
// 
//     for (e, obb, cell_pos, e_over_obbs, _, target_tags) in &e_cells_query {
//         if obb.disabled { continue; }
// 
//         let neighbors = e_quad_map.entity_in_cells(cell_pos);
// 
//         let mut tmp_vec_e: Vec<Entity> = vec![];
// 
//         for other_e in neighbors.unwrap() {
//             if other_e == e { continue; } 
// 
//             if let Ok((_, other_obb, _, _, other_tag, _)) = e_cells_query.get(other_e) {
// 
//                 let mut has_tag = false;
//                 for tag in target_tags.0.iter() {
//                     if tag == &other_tag.0 {
//                         has_tag = true; 
//                     }
//                 }
// 
//                 if has_tag && obb.overlapping(other_obb) {
//                     tmp_vec_e.push(other_e);
//                 }
//             }
//         }
//         tmp_entity_map.insert(e, tmp_vec_e.clone());
//     }
// 
//     for (e, _, _, mut over_obbs, _, _) in &mut e_cells_query {
//         over_obbs.0.clear();
//         if let Some(over_obbs_vec) = tmp_entity_map.get(&e) {
//             over_obbs.0 = over_obbs_vec.clone();
//         }
//     }
// }

pub fn update_entity_overlapping_obbs(
    mut e_cells_query: Query<(Entity, &mut OBB, &CellPos, &mut EntityOverlappingOBBs, &EntityTagContainer, &TargetEntityTags)>,
    e_quad_map: ResMut<EntityQuadMap>,
    mut tmp_entity_map: Local<HashMap<Entity, Vec<Entity>>>,
    mut tmp_vec_e: Local<Vec<Entity>>,
) {
    tmp_entity_map.clear();

    let mut tmp_results: HashMap<Entity, (&OBB, &CellPos, &EntityTagContainer, &TargetEntityTags)> = HashMap::new();

    for (e, obb, cell_pos, _, tag, target_tags) in &e_cells_query {
        tmp_results.insert(e, (obb, cell_pos, tag, target_tags));
    }

    for (e, (obb, cell_pos, _, target_tags)) in &tmp_results {
        if obb.disabled { continue; }

        tmp_vec_e.clear();

        let neighbors = e_quad_map.entity_in_cells(cell_pos);

        for other_e in neighbors.unwrap() {
            if other_e == *e { continue; } 

            if let Some((other_obb, _, other_tag, _)) = tmp_results.get(&other_e) {

                let mut has_tag = false;
                for tag in target_tags.0.iter() {
                    if tag == &other_tag.0 {
                        has_tag = true; 
                    }
                }

                if has_tag && obb.overlapping(other_obb) {
                    tmp_vec_e.push(other_e);
                }
            }
        }
        tmp_entity_map.insert(*e, tmp_vec_e.clone());
    }

    for (e, _, _, mut over_obbs, _, _) in &mut e_cells_query {
        over_obbs.0.clear();
        if let Some(over_obbs_vec) = tmp_entity_map.get(&e) {
            over_obbs.0 = over_obbs_vec.clone();
        }
    }
}
