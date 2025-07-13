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

pub fn physics_fixed_update(ecs: &mut ECS, time_step: f32) {

    let mut area_m = ecs.get_resource_mut::<AreaManager>();

    // let entities = query_entities!(ecs, WalkerData, Velocity, Position, Area);
    for (_e, pos, vel, area, walker_d) in 
        ecs.query_comp::<(&mut Position, &mut Velocity, &mut Area, &mut WalkerData)>() 
    {

        //GRAVITY
        vel.vec.y += GRAVITY_ACCEL;
        if vel.vec.y >= MAX_GRAVITY {
            vel.vec.y = MAX_GRAVITY
        }

        //COLLISION RESOLUTION
        area_colliding_to_tile(
            area, 
            &mut pos.vec, 
            &mut vel.vec, 
            &mut walker_d.grounded, 
            &mut *area_m, 
            time_step
        );

        // needs to happen last due to area colliding grounded trigger not reached due to
        // zeroed velocity
        if walker_d.grounded && vel.vec.y > 0.0 {
            vel.vec.y = 0.0;
        }
        if !walker_d.grounded {
            walker_d.state = WalkerState::Aired;
            // state_m.set_state(State::Aired);
        }

        //ADJUSTMENT
        pos.vec.x += vel.vec.x * time_step;
        pos.vec.y += vel.vec.y * time_step;

        area.update_pos(pos.vec.x, pos.vec.y);
    }
}
