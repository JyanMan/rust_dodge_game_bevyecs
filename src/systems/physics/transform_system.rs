use crate::ecs::ecs::ECS;
use crate::components::{Transform, Owner};

pub fn transform_update_system(ecs: &mut ECS, _time_step: f32) {
    for (_e, pos, owner) in
        ecs.query_comp::<(&mut Transform, &Owner)>() 
    {
        let owner_entity = owner.entity;
        let owner_pos = ecs.get_component::<Transform>(owner_entity).expect("owner has no position component"); 

        pos.global = owner_pos.global + pos.local;
    }
}
