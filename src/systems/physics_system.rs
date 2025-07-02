use std::any::TypeId;
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
        let entities = ecs.query_entities(&[
            TypeId::of::<PlayerData>(),
            TypeId::of::<Velocity>(),
            TypeId::of::<Position>(),
            TypeId::of::<Area>(),
        ]);

        let mut area_m = ecs.get_resource_mut::<AreaManager>();

        for e in entities {
            // let curre_state
            if let (Some(pos), Some(vel), Some(area), Some(p_data)) = (
                ecs.get_component_mut::<Position>(e),
                ecs.get_component_mut::<Velocity>(e),
                ecs.get_component_mut::<Area>(e),
                ecs.get_component_mut::<PlayerData>(e)
            ) {

                //GRAVITY
                vel.y += GRAVITY_ACCEL;
                if vel.y >= MAX_GRAVITY {
                    vel.y = MAX_GRAVITY
                }
                if p_data.grounded && vel.y > 0.0 {
                    vel.y = 0.0;
                }
                area_colliding_to_tile(area, pos, vel, &mut p_data.grounded, &mut *area_m, time_step);

                //COLLISION RESOLUTION

                //ADJUSTMENT
                pos.x += vel.x * time_step;
                pos.y += vel.y * time_step;

                area.update_pos(pos.x, pos.y);
                
            }
        }
    })
}
