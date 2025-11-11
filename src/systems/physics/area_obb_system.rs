use bevy_ecs::prelude::*;
use crate::components::*;

pub fn area_update_system(mut query: Query<(&Transform, &mut Area)>) {
    for (trans, mut area) in &mut query {
        area.update_pos(trans.global.x, trans.global.y);
    }
}

pub fn obb_update_system(mut query: Query<(&Transform, &mut OBB)>) {
    for (trans, mut obb) in &mut query {
        obb.center = trans.global + obb.offset;
        obb.compute_vertices();
    }
}

pub fn obb_collision_system() {

}

