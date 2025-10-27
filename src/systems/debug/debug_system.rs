use bevy_ecs::prelude::*;

use std::any::TypeId;
use crate::core::renderer::*;
use crate::components::Area;
use crate::ecs::ecs::*;
use crate::resources::AreaManager;

pub fn debug_draw_areas_system(ecs: &mut ECS, renderer: &mut Renderer) {
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
}

pub fn debug_draw_entity_areas(world: &mut World, renderer: &mut Renderer) {
    let mut query = world.query::<&Area>();

    for area in query.iter(world) {
        area.draw(renderer);
    }

    let mut area_m = world.get_resource_mut::<AreaManager>().unwrap();
    area_m.draw_tile_areas(renderer);
}
