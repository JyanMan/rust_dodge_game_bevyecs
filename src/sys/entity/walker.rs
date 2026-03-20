use bevy_ecs::prelude::*;
use crate::components::*;

pub fn walker_update(
    mut query: Query<(&Transform, &mut Velocity, &mut WalkerData)>,
) {
    for (trans, mut vel, mut walker_d) in &mut query {
        // let decel: f32 = x_dir.copysign(vel.vec.x) * walker_d.accel;
        // if !walker_d.grounded {
        //     vel.vec.x -= decel * 0.2;
        // }
        // else {
        //     vel.vec.x -= decel;
        // }
        // if vel.vec.x.abs() <= walker_d.accel {
        //     vel.vec.x = 0.0;
        // }
        
    }
}
