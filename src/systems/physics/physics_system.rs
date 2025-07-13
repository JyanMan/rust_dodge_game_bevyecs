use crate::components::area::*;
use crate::components::entity::{ WalkerData, WalkerState };
use crate::components::position::*;
use crate::components::velocity::*;
//use crate::components::state_machine::*;
use crate::core::collision::*;
use crate::systems::entity::player::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::resources::area_manager::*;

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
    for (e, pos, vel, area) in 
        ecs.query_comp::<(&mut Position, &mut Velocity, &mut Area)>() 
    {
        let mut dummy_grounded = false;
        // if not a walker, means entity can fly
        // use dummy grounded to modify pointless bool
        let mut walker_d = ecs.get_component_mut::<WalkerData>(e);
        let grounded: &mut bool = 
            match &mut walker_d {
                Some(wd) => &mut wd.grounded,
                None => &mut dummy_grounded
            };

        area_colliding_to_tile(
            area, 
            &mut pos.vec, 
            &mut vel.vec, 
            grounded, 
            &mut *area_m, 
            time_step
        );

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
    for (_e, pos, vel) in
        ecs.query_comp::<(&mut Position, &Velocity)>()
    {
        //ADJUSTMENT
        pos.vec.x += vel.vec.x * time_step;
        pos.vec.y += vel.vec.y * time_step;
    }
}

pub fn area_update_system(ecs: &mut ECS, _time_step: f32) {
    for (_e, pos, area) in
        ecs.query_comp::<(&Position, &mut Area)>()
    {
        area.update_pos(pos.vec.x, pos.vec.y);
    }
}

pub fn physics_fixed_update(ecs: &mut ECS, time_step: f32) {
    gravity_system(ecs, time_step);
    // GROUNDED STATE CHANGE IS NOT IMMEDIATE
    collision_system(ecs, time_step);
    pos_vel_update_system(ecs, time_step);
    area_update_system(ecs, time_step);
    // let mut area_m = ecs.get_resource_mut::<AreaManager>();
    // for (_e, pos, vel, area, walker_d) in 
    //     ecs.query_comp::<(&mut Position, &mut Velocity, &mut Area, &mut WalkerData)>() 
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
