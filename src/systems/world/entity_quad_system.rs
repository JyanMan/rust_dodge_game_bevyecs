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

pub fn update_entity_overlapping_obbs(
    mut e_cells_query: Query<(Entity, &mut OBB, &CellPos, &mut EntityOverlappingOBBs, &EntityTagContainer)>,
    e_quad_map: ResMut<EntityQuadMap>,
    mut tmp_over_e: Local<Vec<(Entity, Vec<Entity>)>>,
) {
    tmp_over_e.clear();
    let mut q: HashMap<Entity, Vec<Entity>> = HashMap::new();

    for (e, obb, cell_pos, _, e_tag_cont) in &e_cells_query {
        let neighbors = e_quad_map.entity_in_cells(cell_pos);

        let mut tmp_vec_e: Vec<Entity> = vec![];

        for other_e in neighbors.unwrap() {
            if other_e == e { continue; } 

            if let Ok((_, other_obb, _, other_over_obbs, _)) = e_cells_query.get(other_e) {
                let mut has_tag = false;
                for tag in other_over_obbs.target_tags.iter() {
                    if tag == &e_tag_cont.0 {
                        has_tag = true; 
                    }
                }

                if !has_tag {
                    continue;
                }

                if obb.overlapping(other_obb) {
                    tmp_vec_e.push(other_e);
                }
            }
        }
        q.insert(e, tmp_vec_e.clone());
        // tmp_over_e.push((e, tmp_vec_e));
    }

    for (e, _, _, mut other_over_obbs, _) in &mut e_cells_query {
        if let Some(over_obbs_vec) = q.get(&e) {
            other_over_obbs.entities = over_obbs_vec.clone();
        }
    }

    // for (e, other_vec_e) in tmp_over_e.iter() {
    //     if let Ok((_, _, _, mut e_over_obb, _)) = e_cells_query.get_mut(*e) {
    //         e_over_obb.entities = other_vec_e.clone();
    //     }
    // }
}
