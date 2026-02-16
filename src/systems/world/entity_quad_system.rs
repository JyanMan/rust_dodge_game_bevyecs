use crate::components::*;
use crate::resources::*;
use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
// use xsparseset::*;

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

#[derive(Clone)]
pub struct LastChecked(i32);

type EOverResult = (Entity, OBB, EntityTagContainer, LastChecked);
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
    mut tmp_entity_set: Local<SparseSet<Entity, EntityTagVec>>,
    mut tmp_result_set: Local<SparseSet<Entity, EOverResult>>
) {
    tmp_entity_set.clear();
    tmp_result_set.clear();

    for (e, obb, _, _, tag, _) in &e_cells_query {
        tmp_result_set.insert(e, (e, obb.clone(), tag.clone(), LastChecked(0)));
    }
    let mut frame: i32 = 1;

    for (e, obb, cell_pos, mut over_obbs, _, target_tags) in &mut e_cells_query {
        if obb.disabled || target_tags.0.is_empty() {
            continue;
        }

        over_obbs.0.clear();

        let neighbors = e_quad_map.entity_in_cells(cell_pos);

        for other_e in neighbors {
            if other_e == e {
                continue;
            }

            if let Some((_, other_obb, other_tag, last_check)) = tmp_result_set.get_mut(other_e) {
                if other_obb.disabled {
                    continue;
                }

                // skip duplicates
                if last_check.0 == frame { continue; }
                last_check.0 = frame;

                // filter target tags
                let mut has_tag = false;
                for tag in target_tags.0.iter() {
                    if tag == &other_tag.0 {
                        has_tag = true;
                    }
                }

                // add overlapping obbs
                if has_tag && obb.overlapping(other_obb) {
                    over_obbs.0.insert(other_e, other_tag.0.clone());
                }
            }
        }

        frame += 1;
    }

}
