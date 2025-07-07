use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::ecs::ecs_query::*;
use crate::managers::chunk_manager::*;
use crate::managers::area_manager::*;
use crate::systems::player_system::*;

pub fn chunk_startup_system() -> StartFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        ecs.add_resource::<ChunkManager>(ChunkManager::new(
             Position::new(0.0, 0.0),
             &renderer.asset_m,
             4,
        ));
    })
}

pub fn chunk_update_system() -> UpdateFn {
    Box::new(|ecs: &mut ECS, _delta_time: f32| {
        let mut chunk_m = ecs.get_resource_mut::<ChunkManager>();
        let mut area_m = ecs.get_resource_mut::<AreaManager>();

        for (_p_tag, p_pos) in
            ecs.query_comp::<(&PlayerTag, &Position)>() {
            chunk_m.generate(*p_pos, &mut *area_m);
        }
    }) 
}

pub fn chunk_draw_system() -> DrawFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let mut chunk_m = ecs.get_resource_mut::<ChunkManager>();
        chunk_m.draw(renderer);
    }) 
}
