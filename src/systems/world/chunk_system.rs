use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::components::Vector2;
use crate::components::entity::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::ecs::ecs_query::*;
use crate::managers::chunk_manager::*;
use crate::managers::area_manager::*;
use crate::systems::entity::player::*;

pub fn chunk_manager_init(ecs: &mut ECS, renderer: &mut Renderer) {
    ecs.add_resource::<ChunkManager>(ChunkManager::new(
         Vector2::new(0.0, 0.0),
         &renderer.asset_m,
         4,
    ));
}

pub fn chunk_manager_update(ecs: &mut ECS, _delta_time: f32) {
    let mut chunk_m = ecs.get_resource_mut::<ChunkManager>();
    let mut area_m = ecs.get_resource_mut::<AreaManager>();

    for (_e, _p_data, p_pos) in
        ecs.query_comp::<(&PlayerData, &Position)>() {
        chunk_m.generate(&p_pos.vec, &mut *area_m);
    }
}

pub fn chunk_manager_draw(ecs: &mut ECS, renderer: &mut Renderer) {
    let mut chunk_m = ecs.get_resource_mut::<ChunkManager>();
    chunk_m.draw(renderer);
}
