use bevy_ecs::prelude::*;
use crate::components::*;

pub fn area_update(mut query: Query<(&Transform, &mut Area)>) {
    query.par_iter_mut().for_each(|(trans, mut area)| {
        area.update_pos(trans.pos.x, trans.pos.y);
    });
}

pub fn obb_update(
    mut query: Query<(&Transform, &LocalTransform, &mut OBB)>
) {
    query.par_iter_mut().for_each(|(trans, local, mut obb)| {
        obb.center = trans.pos + obb.offset;

        // TODO: this does not work as an actual flip, the object will just fix its rotation
        // still only facing the 1st and 4th quadrant
        if trans.scale.x < 0.0 {
            obb.rotation = -local.rot;
        }
        else {
            obb.rotation = local.rot;
        }
        obb.compute_vertices();
    });
}

