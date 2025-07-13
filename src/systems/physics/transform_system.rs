use crate::ecs::ecs::ECS;
use crate::components::{Position, Owner};

pub fn transform_update_system(ecs: &mut ECS, _time_step: f32) {
    for (_e, pos, owner) in
        ecs.query_comp::<(&mut Position, &Owner)>() 
    {
        let owner_entity = owner.entity;
        let owner_pos = ecs.get_component::<Position>(owner_entity).expect("owner has no position component"); 

        pos.vec = owner_pos.vec + pos.local;
    }
}
