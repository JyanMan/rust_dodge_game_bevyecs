use bevy_ecs::prelude::*;
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
){
    for transform in &query {
        e_quad_map.generate(&transform.global);
    }
}
