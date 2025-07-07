use crate::components::area::*;
use crate::components::position::*;
use crate::components::velocity::*;
use crate::core::collision::*;
use crate::systems::player_system::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::area_manager::*;

const GRAVITY_ACCEL: f32 = 20.0;
const MAX_GRAVITY: f32 = 700.0;

pub fn physics_fixed_update_system() -> FixedUpdateFn {
    Box::new(|ecs: &mut ECS, time_step: f32| {

        let mut area_m = ecs.get_resource_mut::<AreaManager>();

        // let entities = query_entities!(ecs, PlayerData, Velocity, Position, Area);
        for (pos, vel, area, p_data) in 
            ecs.query_comp::<(&mut Position, &mut Velocity, &mut Area, &mut PlayerData)>() {

            //GRAVITY
            vel.y += GRAVITY_ACCEL;
            if vel.y >= MAX_GRAVITY {
                vel.y = MAX_GRAVITY
            }

            //COLLISION RESOLUTION
            area_colliding_to_tile(area, pos, vel, &mut p_data.grounded, &mut *area_m, time_step);

            // needs to happen last due to area colliding grounded trigger not reached due to
            // zeroed velocity
            if p_data.grounded && vel.y > 0.0 {
                vel.y = 0.0;
            }


            //ADJUSTMENT
            pos.x += vel.x * time_step;
            pos.y += vel.y * time_step;

            area.update_pos(pos.x, pos.y);
        }
    })
}
