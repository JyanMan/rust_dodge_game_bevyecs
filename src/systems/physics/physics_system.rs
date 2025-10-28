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
const MAX_GRAVITY: f32 = 700.0;

pub fn gravity_system(mut query: Query<(&mut Velocity, &GravityAffected)>) {
    for (mut vel, grav_affected) in &mut query {
        if grav_affected.0 == false {
            continue;
        }
        //GRAVITY
        vel.vec.y += GRAVITY_ACCEL;
        if vel.vec.y >= MAX_GRAVITY {
            vel.vec.y = MAX_GRAVITY
        }
    }
}

pub fn collision_system(
    mut area_m: ResMut<AreaManager>, 
    ts_res: Res<TimeStepRes>, 
    mut query: Query<(&mut Transform, &mut Velocity, &mut Area, Option<&mut WalkerData>)>
) {
    // let mut area_m = ecs.get_resource_mut::<AreaManager>();
    for (mut trans, mut vel, mut area, mut walker_d) in &mut query {
        // if not a walker, means entity can fly
        // use dummy grounded to modify pointless bool
        let mut dummy_grounded = false;

        let grounded: &mut bool = 
            match &mut walker_d {
                Some(wd) => &mut wd.grounded,
                None => &mut dummy_grounded
            };

        // COLLISION RESOLUTION
        area_colliding_to_tile(
            &mut area, 
            &mut trans.global, 
            &mut vel.vec, 
            grounded, 
            &mut *area_m, 
            ts_res.time_step
        );

        // only walkers have grounded state
        match &mut walker_d {
            Some(wd) => {
                if !wd.grounded {
                    wd.state = WalkerState::Aired;
                }
            },
            None => {}
        };
    }
}

pub fn pos_vel_update_system(mut query: Query<(&mut Transform, &Velocity)>, ts_res: Res<TimeStepRes>) {
    for (mut trans, vel) in &mut query {
        trans.global.x += vel.vec.x * ts_res.time_step;
        trans.global.y += vel.vec.y * ts_res.time_step;
    }
}

pub fn area_update_system(mut query: Query<(&Transform, &mut Area)>) {
    for (trans, mut area) in &mut query {
        area.update_pos(trans.global.x, trans.global.y);
    }
}

pub fn physics_fixed_update(ecs: &mut ECS, time_step: f32) {
    // gravity_system(ecs, time_step);
    // collision_system(ecs, time_step);
    // pos_vel_update_system(ecs, time_step);
    // transform_update_system(ecs, time_step);
    // area_update_system(ecs, time_step);
    
    // let mut area_m = ecs.get_resource_mut::<AreaManager>();
    // for (_e, pos, vel, area, walker_d) in 
    //     ecs.query_comp::<(&mut Transform, &mut Velocity, &mut Area, &mut WalkerData)>() 
    // {

    //     //GRAVITY
    //     vel.vec.y += GRAVITY_ACCEL;
    //     if vel.vec.y >= MAX_GRAVITY {
    //         vel.vec.y = MAX_GRAVITY
    //     }

    //     //COLLISION RESOLUTION
    //     area_colliding_to_tile(
    //         area, 
    //         &mut pos.vec, 
    //         &mut vel.vec, 
    //         &mut walker_d.grounded, 
    //         &mut *area_m, 
    //         time_step
    //     );

    //     // needs to happen last due to area colliding grounded trigger not reached due when
    //     // zeroed velocity
    //     if walker_d.grounded && vel.vec.y > 0.0 {
    //         vel.vec.y = 0.0;
    //     }
    //     if !walker_d.grounded {
    //         walker_d.state = WalkerState::Aired;
    //         // state_m.set_state(State::Aired);
    //     }

    //     //ADJUSTMENT
    //     pos.vec.x += vel.vec.x * time_step;
    //     pos.vec.y += vel.vec.y * time_step;

    //     area.update_pos(pos.vec.x, pos.vec.y);
    // }
}
