use bevy_ecs::prelude::*;
use crate::components::*;

pub fn area_update(mut query: Query<(&Transform, &mut Area)>) {
    query.par_iter_mut().for_each(|(trans, mut area)| {
        area.update_pos(trans.pos.x, trans.pos.y);
    });
}

pub fn obb_update(mut query: Query<(&Transform, &mut OBB)>) {
    query.par_iter_mut().for_each(|(trans, mut obb)| {
        obb.center = trans.pos + obb.offset;
        obb.compute_vertices();
    });
}

