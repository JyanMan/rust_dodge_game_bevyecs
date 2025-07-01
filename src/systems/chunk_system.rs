use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::chunk_manager::*;
use crate::managers::area_manager::*;
use crate::systems::player_system::*;

pub fn chunk_startup_system() -> StartFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        ecs.add_resource::<ChunkManager>(ChunkManager::new(
             Position::new(0.0, 0.0),
             &renderer.asset_m,
             2,
        ));
    })
}

pub fn chunk_update_system() -> UpdateFn {
    Box::new(|ecs: &mut ECS, _delta_time: f32| {
        let players = ecs.query_entities(&[
            TypeId::of::<PlayerTag>(),
            TypeId::of::<Position>(),
        ]);
        let mut chunk_m = ecs.get_resource_mut::<ChunkManager>().expect("failed resource");
        for e in players {
            if let (Some(p_pos), Some(_p_tab)) = (
                ecs.get_component::<Position>(e),
                ecs.get_component::<PlayerTag>(e)
            ) {
                chunk_m.generate(*p_pos);
            }
                   
        }
    }) 
}

pub fn chunk_draw_system() -> DrawFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let mut chunk_m = ecs.get_resource_mut::<ChunkManager>().expect("failed to get resource");
        chunk_m.draw(renderer);
        // let entities = ecs.query_entities(&[
        //     TypeId::of::<ChunkManager>(),
        // ]);

        // for e in entities {
        //     if let Some(chunk_m) = ecs.get_component_mut::<ChunkManager>(e) {
        //         chunk_m.draw(renderer);
        //     }
        // }
    }) 
}
