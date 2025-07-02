use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::area::*;
use crate::ecs::system::*;
use crate::ecs::ecs::*;
use crate::managers::area_manager::*;
use crate::systems::player_system::*;

pub fn debug_draw_areas_system() -> DrawFn {
    Box::new(|ecs: &mut ECS, renderer: &mut Renderer| {
        let entities = ecs.query_entities(&[
            TypeId::of::<Area>(),
        ]);

        for e in entities {
            if let Some(area) = ecs.get_component_mut::<Area>(e) {
                area.draw(renderer);
            }
        }

        let mut area_m = ecs.get_resource_mut::<AreaManager>();
        area_m.draw_tile_areas(renderer);
    })
}
