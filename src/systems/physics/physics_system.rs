use crate::components::area::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::{ Transform, Velocity };
use crate::core::collision::*;
use crate::ecs::ecs::*;
use crate::resources::area_manager::*;
use super::*;

const GRAVITY_ACCEL: f32 = 15.0;
const MAX_GRAVITY: f32 = 700.0;

pub fn gravity_system(ecs: &mut ECS, _time_step: f32) {
    for (_e, vel, _walker_d) in 
        ecs.query_comp::<(&mut Velocity,  &mut WalkerData)>() 
    {
        //GRAVITY
        vel.vec.y += GRAVITY_ACCEL;
        if vel.vec.y >= MAX_GRAVITY {
            vel.vec.y = MAX_GRAVITY
        }
    }
}

pub fn collision_system(ecs: &mut ECS, time_step: f32) {
    let mut area_m = ecs.get_resource_mut::<AreaManager>();
    for (e, trans, vel, area) in 
        ecs.query_comp::<(&mut Transform, &mut Velocity, &mut Area)>() 
    {
        // if not a walker, means entity can fly
        // use dummy grounded to modify pointless bool
        let mut dummy_grounded = false;
        let mut walker_d = ecs.get_component_mut::<WalkerData>(e);

        let grounded: &mut bool = 
            match &mut walker_d {
                Some(wd) => &mut wd.grounded,
                None => &mut dummy_grounded
            };

        // COLLISION RESOLUTION
        area_colliding_to_tile(
            area, 
            &mut trans.global, 
            &mut vel.vec, 
            grounded, 
            &mut *area_m, 
            time_step
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

pub fn pos_vel_update_system(ecs: &mut ECS, time_step: f32) {
    for (_e, trans, vel) in
        ecs.query_comp::<(&mut Transform, &Velocity)>()
    {
        trans.global.x += vel.vec.x * time_step;
        trans.global.y += vel.vec.y * time_step;
    }
}

pub fn area_update_system(ecs: &mut ECS, _time_step: f32) {
    for (_e, trans, area) in
        ecs.query_comp::<(&Transform, &mut Area)>()
    {
        area.update_pos(trans.global.x, trans.global.y);
    }
}

pub fn physics_fixed_update(ecs: &mut ECS, time_step: f32) {
    gravity_system(ecs, time_step);
    collision_system(ecs, time_step);
    pos_vel_update_system(ecs, time_step);
    transform_update_system(ecs, time_step);
    area_update_system(ecs, time_step);
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
