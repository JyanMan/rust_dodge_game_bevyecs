use crate::components::*;
use crate::resources::*;
use bevy_ecs::prelude::*;
use bevy_ecs::storage::SparseSet;
// use xsparseset::*;

/* PHYSICS */
pub fn update(
    mut e_quad_map: ResMut<EntityQuadMap>,
    mut query: Query<(Entity, &mut CellPos, &OBB)>,
) {
    for (e, mut cell_pos, obb) in &mut query {
        e_quad_map.update_entity_cell(e, &mut cell_pos, obb);
    }
}

pub fn generate(
    mut e_quad_map: ResMut<EntityQuadMap>,
    query: Query<&Transform, With<PlayerTag>>,
) {
    for transform in &query {
        e_quad_map.generate(&transform.global);
    }
}

#[derive(Clone)]
pub struct LastChecked(i32);

type EOverResult = (Entity, OBB, LastChecked);
// type EntityTagVec = Vec<(Entity, Entity)>;

pub fn update_overlapping_obbs(
    mut e_cells_query: Query<(
        Entity,
        &OBB,
        &CellPos,
        &mut EntityOverlappingOBBs,
        // &EntityTagContainer,
        &TargetEntityTags,
    )>,
    e_quad_map: ResMut<EntityQuadMap>,
    tag_reg: Res<TagRegistry>,
    // mut tmp_entity_set: Local<SparseSet<Entity, EntityTagVec>>,
    mut tmp_result_set: Local<SparseSet<Entity, EOverResult>>
) {
    // tmp_entity_set.clear();
    tmp_result_set.clear();

    for (e, obb, _, _,  _) in &e_cells_query {
        tmp_result_set.insert(e, (e, obb.clone(), LastChecked(0)));
    }
    let mut frame: i32 = 1;

    for (e, obb, cell_pos, mut over_obbs, target_tags) in &mut e_cells_query {
        if obb.disabled || target_tags.0.is_empty() {
            continue;
        }

        over_obbs.0.clear();

        let neighbors = e_quad_map.entity_in_cells(cell_pos);

        for other_e in neighbors {
            if other_e == e {
                continue;
            }

            if let Some((_, other_obb, last_check)) = tmp_result_set.get_mut(other_e) {
                if other_obb.disabled {
                    continue;
                }

                // skip duplicates
                if last_check.0 == frame { continue; }
                last_check.0 = frame;

                // filter target tags
                let mut has_tag = false;
                for tag in target_tags.0.iter() {
                    if tag_reg.entity_contains_tag_id(other_e, *tag) {
                        // println!("found it");
                        has_tag = true;
                    }
                    // if tag == &other_tag.0 {
                    //     has_tag = true;
                    // }
                }

                // add overlapping obbs
                if has_tag && obb.overlapping(other_obb) {
                    over_obbs.0.insert(other_e, other_e);
                }
            }
        }

        frame += 1;
    }

}

