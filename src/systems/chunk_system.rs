use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::position::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::chunk_manager::*;
use crate::systems::player_system::*;

pub fn chunk_startup_system() -> StartFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let chunk_m = ecs.create_entity();
        ecs.register_component::<ChunkManager>();

        ecs.add_component::<ChunkManager>(chunk_m, ChunkManager::new(
            Position::new(0.0, 0.0),
            &renderer.asset_m,
            3,
        ));
    })
}

pub fn chunk_update_system() -> UpdateFn {
    Box::new(|ecs: &mut ECS, _delta_time: f32| {
        let players = ecs.query_entities(&[
            TypeId::of::<PlayerTag>(),
            TypeId::of::<Position>(),
        ]);
        let chunk_managers = ecs.query_entities(&[TypeId::of::<ChunkManager>()]);

        for e in players {
            if let (Some(p_pos), Some(_p_tab)) = (
                ecs.get_component::<Position>(e),
                ecs.get_component::<PlayerTag>(e)
            ) {
                let player_pos = p_pos.clone();
                for cm in chunk_managers.iter() {
                    if let Some(chunk_m) = ecs.get_component_mut::<ChunkManager>(*cm) {
                        chunk_m.generate(player_pos);
                    }
                }
            }
        }
    }) 
}

pub fn chunk_draw_system() -> DrawFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let entities = ecs.query_entities(&[
            TypeId::of::<ChunkManager>(),
        ]);

        for e in entities {
            if let Some(chunk_m) = ecs.get_component_mut::<ChunkManager>(e) {
                chunk_m.draw(renderer);
            }
        }
    }) 
}
