use bevy_ecs::prelude::*;

use crate::components::GravityAffected;
use crate::components::area::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::{ Transform, Velocity };
use crate::core::collision::*;
use crate::ecs::ecs::*;
use crate::resources::area_manager::*;
use crate::resources::*;

const GRAVITY_ACCEL: f32 = 15.0;
const MAX_GRAVITY: f32 = 500.0;

pub fn gravity_system(mut query: Query<(&mut Velocity, &GravityAffected)>) {
    query.par_iter_mut().for_each(|(mut vel, grav_affected)| {
        if grav_affected.0 == false {
            return;
        }
        //GRAVITY
        vel.vec.y += GRAVITY_ACCEL;
        if vel.vec.y >= MAX_GRAVITY {
            vel.vec.y = MAX_GRAVITY
        }
    });
}

pub fn collision_system(
    area_m: Res<AreaManager>, 
    time_step: Res<TimeStep>, 
    mut query: Query<(&mut Transform, &mut Velocity, &mut Area, Option<&mut WalkerData>)>
) {
    // let mut area_m = ecs.get_resource_mut::<AreaManager>();
    query.par_iter_mut().for_each(|(mut trans, mut vel, mut area, mut walker_d)| {
        let mut dummy_grounded = false;
        let grounded: &mut bool = match &mut walker_d {
            Some(wd) => &mut wd.grounded,
            None => &mut dummy_grounded
        };

        // pass area_m as &AreaManager
        area_colliding_to_tile(&mut area, &mut trans.global, &mut vel.vec, grounded, &area_m, time_step.0);

        if let Some(wd) = &mut walker_d {
            if !wd.grounded {
                wd.state = WalkerState::Aired;
            }
        }
    });
}

pub fn pos_vel_update_system(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<TimeStep>) {
    query.par_iter_mut().for_each(|(mut trans, vel)| {
        trans.global.x += vel.vec.x * time_step.0;
        trans.global.y += vel.vec.y * time_step.0;
    });
}
