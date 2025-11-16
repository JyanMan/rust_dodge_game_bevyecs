use bevy_ecs::prelude::*;

use crate::components::GravityAffected;
use crate::components::area::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::{ Transform, Velocity };
use crate::core::collision::*;
use crate::resources::area_manager::*;
use crate::resources::*;

const GRAVITY_ACCEL: f32 = 15.0;
const MAX_GRAVITY: f32 = 500.0;

pub fn gravity_system(mut query: Query<(&mut Velocity, &GravityAffected)>) {
    query.par_iter_mut().for_each(|(mut vel, grav_affected)| {
        if !grav_affected.0 {
            return;
        }
        //GRAVITY
        vel.vec.y += GRAVITY_ACCEL;
        if vel.vec.y >= MAX_GRAVITY {
            vel.vec.y = MAX_GRAVITY
        }
    });
}

pub fn walker_collision_system(
    area_m: Res<AreaManager>, 
    time_step: Res<TimeStep>, 
    mut query: Query<(&mut Transform, &mut Velocity, &mut Area, &mut WalkerData)>
) {
    // let mut area_m = ecs.get_resource_mut::<AreaManager>();
    query.par_iter_mut().for_each(|(mut trans, mut vel, mut area, mut walker_d)| {

        // pass area_m as &AreaManager
        area_colliding_to_tile(&mut area, &mut trans.global, &mut vel.vec, &mut walker_d.grounded, &area_m, time_step.0);

        if !walker_d.grounded {
            walker_d.state = WalkerState::Aired;
        }
    });
}

pub fn pos_vel_update_system(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<TimeStep>) {
    query.par_iter_mut().for_each(|(mut trans, vel)| {
        trans.global.x += vel.vec.x * time_step.0;
        trans.global.y += vel.vec.y * time_step.0;
    });
}
