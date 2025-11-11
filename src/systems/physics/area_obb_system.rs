use bevy_ecs::prelude::*;
use crate::components::*;

pub fn area_update_system(mut query: Query<(&Transform, &mut Area)>) {
    query.par_iter_mut().for_each(|(trans, mut area)| {
        area.update_pos(trans.global.x, trans.global.y);
    });
}

pub fn obb_update_system(mut query: Query<(&Transform, &mut OBB)>) {
    query.par_iter_mut().for_each(|(trans, mut obb)| {
        obb.center = trans.global + obb.offset;
        obb.compute_vertices();
    });
}

