use crate::components::*;
use crate::resources::*;
use bevy_ecs::prelude::*;
use std::collections::HashMap;
use xsparseset::*;

/* PHYSICS */
pub fn update_entity_quad_system(
    mut e_quad_map: ResMut<EntityQuadMap>,
    mut query: Query<(Entity, &mut CellPos, &OBB)>,
) {
    for (e, mut cell_pos, obb) in &mut query {
        e_quad_map.update_entity_cell(e, &mut cell_pos, obb);
    }
}

pub fn quad_generation_system(
    mut e_quad_map: ResMut<EntityQuadMap>,
    query: Query<&Transform, With<PlayerTag>>,
) {
    for transform in &query {
        e_quad_map.generate(&transform.global);
    }
}

type EOverResult <'a> = (Entity, &'a OBB, &'a CellPos, &'a EntityTagContainer, &'a TargetEntityTags);
type EntityTagVec = Vec<(Entity, EntityTag)>;

pub fn update_entity_overlapping_obbs(
    mut e_cells_query: Query<(
        Entity,
        &mut OBB,
        &CellPos,
        &mut EntityOverlappingOBBs,
        &EntityTagContainer,
        &TargetEntityTags,
    )>,
    e_quad_map: ResMut<EntityQuadMap>,
    mut tmp_entity_set: Local<SparseSet<usize, EntityTagVec, VecStorage<usize>>>,
    mut tmp_vec_e: Local<EntityTagVec>,
) {
    tmp_entity_set.clear();

    let mut tmp_result_set: SparseSet<usize, EOverResult, VecStorage<usize>> = SparseSet::default();

    for (e, obb, cell_pos, _, tag, target_tags) in &e_cells_query {
        tmp_result_set.insert(e.index() as usize, (e, obb, cell_pos, tag, target_tags));
    }

    for (e, obb, cell_pos, _, target_tags) in tmp_result_set.data() {
        if obb.disabled {
            continue;
        }

        tmp_vec_e.clear();

        let neighbors = e_quad_map.entity_in_cells(cell_pos);

        for other_e in neighbors.unwrap() {
            if other_e == *e {
                continue;
            }

            if let Some((_, other_obb, _, other_tag, _)) = tmp_result_set.get(other_e.index() as usize) {
                if other_obb.disabled {
                    continue;
                }

                let mut has_tag = false;
                for tag in target_tags.0.iter() {
                    if tag == &other_tag.0 {
                        has_tag = true;
                    }
                }

                if has_tag && obb.overlapping(other_obb) {
                    // println!("asdfa");
                    tmp_vec_e.push((other_e, other_tag.0.clone()));
                }
            }
        }
        if !tmp_vec_e.is_empty() {
            tmp_entity_set.insert(e.index() as usize, tmp_vec_e.clone());
        }
    }

    for (e, _, _, mut over_obbs, _, _) in &mut e_cells_query {
        if let Some(over_obbs_vec) = tmp_entity_set.get(e.index() as usize) {
            over_obbs.0 = over_obbs_vec.clone();
            continue;
        }
        over_obbs.0.clear();
    }
}
