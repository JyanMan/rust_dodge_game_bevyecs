use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::Transform;
use crate::components::entity::*;
use crate::ecs::ecs::*;
use crate::resources::chunk_manager::*;
use crate::resources::area_manager::*;

// pub fn chunk_manager_init(ecs: &mut ECS, renderer: &mut Renderer) {
//     ecs.add_resource::<ChunkManager>(ChunkManager::new(
//          Vector2::new(0.0, 0.0),
//          &renderer.asset_m,
//          4,
//     ));
// }

pub fn chunk_manager_update(ecs: &mut ECS, _delta_time: f32) {
    let mut chunk_m = ecs.get_resource_mut::<ChunkManager>();
    let mut area_m = ecs.get_resource_mut::<AreaManager>();

    for (_e, _p_data, p_trans) in
        ecs.query_comp::<(&PlayerData, &Transform)>() 
    {
        chunk_m.generate(&p_trans.global, &mut *area_m);
    }
}

pub fn chunk_manager_draw(ecs: &mut ECS, renderer: &mut Renderer) {
    let mut chunk_m = ecs.get_resource_mut::<ChunkManager>();
    chunk_m.draw(renderer);
}
