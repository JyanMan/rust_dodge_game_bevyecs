use crate::components::*;
use crate::resources::*;
use bevy_ecs::prelude::*;
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

struct LastChecked(i32);

type EOverResult <'a> = (Entity, &'a OBB, &'a CellPos, &'a EntityTagContainer, &'a TargetEntityTags, LastChecked);
type EntityTagVec = Vec<(Entity, EntityTag)>;

pub fn update_entity_overlapping_obbs(
    mut e_cells_query: Query<(
        Entity,
        &OBB,
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
    let query_size = e_cells_query.iter().len();
    // preallocate to allow batch insertion
    let mut entities: Vec<usize> = Vec::with_capacity(query_size);
    let mut e_immuts: Vec<EOverResult> = Vec::with_capacity(query_size);

    for (e, obb, cell_pos, _, tag, target_tags) in &e_cells_query {
        // tmp_result_set.insert(e.index() as usize, (e, obb, cell_pos, tag, target_tags, LastChecked(0)));
        entities.push(e.index() as usize);
        e_immuts.push((e, obb, cell_pos, tag, target_tags, LastChecked(0)));
    }
    tmp_result_set.insert_batch(&mut entities, &mut e_immuts);

    // used to track duplicate
    let mut frame: i32 = 1;

    for (e, obb, cell_pos, _, target_tags, _) in tmp_result_set.data() {
        if obb.disabled || target_tags.0.is_empty() {
            continue;
        }

        tmp_vec_e.clear();

        let neighbors = e_quad_map.entity_in_cells(cell_pos);

        for other_e in neighbors {
            if other_e == *e {
                continue;
            }

            if let Some((_, other_obb, _, other_tag, _, last_check)) = tmp_result_set.get(other_e.index() as usize) {
                if other_obb.disabled {
                    continue;
                }

                // skip duplicates
                if last_check.0 == frame { continue; }
                // UNSAFE: increment counter to prevent future duplicate
                unsafe {
                    let raw_ptr: *const LastChecked = last_check;
                    let raw_mut: *mut LastChecked = raw_ptr as *mut LastChecked;
                    (*raw_mut).0 = frame;
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

        frame += 1;
    }

    for (e, _, _, mut over_obbs, _, _) in &mut e_cells_query {
        over_obbs.0.clear();
        if let Some(over_obbs_vec) = tmp_entity_set.get(e.index() as usize) {
            for (e, tag) in over_obbs_vec.iter() {
                over_obbs.0.push((*e, tag.clone()));
            }
            // over_obbs.0 = over_obbs_vec.clone();
        }
    }
}
